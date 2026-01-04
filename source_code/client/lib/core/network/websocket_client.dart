import 'dart:async';
import 'dart:convert';

import 'package:web_socket_channel/web_socket_channel.dart';

import '../config/app_config.dart';
import '../di/injection.dart';
import '../storage/session_manager.dart';

class WebSocketClient {
  WebSocketChannel? _channel;
  final _messageController = StreamController<Map<String, dynamic>>.broadcast();
  Timer? _heartbeatTimer;
  bool _isConnected = false;

  Stream<Map<String, dynamic>> get messages => _messageController.stream;
  bool get isConnected => _isConnected;

  Future<void> connect() async {
    if (_isConnected) return;

    final token = await getIt<SessionManager>().getAccessToken();
    if (token == null) return;

    final uri = Uri.parse('${AppConfig.wsBaseUrl}/gateway');
    _channel = WebSocketChannel.connect(uri);

    _channel!.stream.listen(
      (data) {
        final message = jsonDecode(data as String) as Map<String, dynamic>;
        _handleMessage(message);
        _messageController.add(message);
      },
      onError: (error) {
        _isConnected = false;
        _reconnect();
      },
      onDone: () {
        _isConnected = false;
        _reconnect();
      },
    );

    // Send identify
    _send({
      'op': 0,
      'd': {'token': token},
    });

    _isConnected = true;
    _startHeartbeat();
  }

  void _handleMessage(Map<String, dynamic> message) {
    final op = message['op'] as int?;
    if (op == 0 && message['t'] == 'READY') {
      // Connected successfully
    }
  }

  void _startHeartbeat() {
    _heartbeatTimer?.cancel();
    _heartbeatTimer = Timer.periodic(
      const Duration(seconds: 30),
      (_) => _send({'op': 1, 'd': null}),
    );
  }

  void _reconnect() async {
    _heartbeatTimer?.cancel();
    await Future.delayed(const Duration(seconds: 5));
    connect();
  }

  void subscribe(String channelId) {
    _send({
      'op': 3,
      'd': {'channel_id': channelId},
    });
  }

  void unsubscribe(String channelId) {
    _send({
      'op': 4,
      'd': {'channel_id': channelId},
    });
  }

  void sendTyping(String channelId) {
    _send({
      'op': 5,
      'd': {'channel_id': channelId},
    });
  }

  void _send(Map<String, dynamic> data) {
    _channel?.sink.add(jsonEncode(data));
  }

  void disconnect() {
    _heartbeatTimer?.cancel();
    _channel?.sink.close();
    _isConnected = false;
  }

  void dispose() {
    disconnect();
    _messageController.close();
  }
}
