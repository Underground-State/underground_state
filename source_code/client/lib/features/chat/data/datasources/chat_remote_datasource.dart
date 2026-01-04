import 'dart:async';

import '../../../../core/network/api_client.dart';
import '../../../../core/network/websocket_client.dart';
import '../models/message_model.dart';

abstract class ChatRemoteDataSource {
  Future<List<MessageModel>> getMessages(
    String channelId, {
    String? before,
    int limit = 50,
  });

  Future<MessageModel> sendMessage(String channelId, String content);
  Future<MessageModel> editMessage(String channelId, String messageId, String content);
  Future<void> deleteMessage(String channelId, String messageId);
  Future<void> addReaction(String channelId, String messageId, String emoji);
  Future<void> removeReaction(String channelId, String messageId, String emoji);

  Stream<MessageModel> get messageStream;
  void subscribeToChannel(String channelId);
  void unsubscribeFromChannel(String channelId);
}

class ChatRemoteDataSourceImpl implements ChatRemoteDataSource {
  final ApiClient _client;
  final WebSocketClient _wsClient;
  final _messageController = StreamController<MessageModel>.broadcast();
  final Set<String> _subscribedChannels = {};

  ChatRemoteDataSourceImpl(this._client, this._wsClient) {
    _wsClient.messages.listen(_handleWebSocketMessage);
  }

  void _handleWebSocketMessage(Map<String, dynamic> data) {
    final eventType = data['t'] as String?;
    final eventData = data['d'] as Map<String, dynamic>?;

    if (eventData == null) return;

    switch (eventType) {
      case 'MESSAGE_CREATE':
      case 'MESSAGE_UPDATE':
        final channelId = eventData['channel_id'] as String?;
        if (channelId != null && _subscribedChannels.contains(channelId)) {
          _messageController.add(MessageModel.fromJson(eventData));
        }
        break;
    }
  }

  @override
  Stream<MessageModel> get messageStream => _messageController.stream;

  @override
  void subscribeToChannel(String channelId) {
    _subscribedChannels.add(channelId);
  }

  @override
  void unsubscribeFromChannel(String channelId) {
    _subscribedChannels.remove(channelId);
  }

  @override
  Future<List<MessageModel>> getMessages(
    String channelId, {
    String? before,
    int limit = 50,
  }) async {
    final queryParams = <String, dynamic>{
      'limit': limit,
      if (before != null) 'before': before,
    };

    final response = await _client.get<List<dynamic>>(
      '/channels/$channelId/messages',
      queryParameters: queryParams,
    );

    return (response.data ?? [])
        .map((json) => MessageModel.fromJson(json as Map<String, dynamic>))
        .toList();
  }

  @override
  Future<MessageModel> sendMessage(String channelId, String content) async {
    final response = await _client.post<Map<String, dynamic>>(
      '/channels/$channelId/messages',
      data: {'content': content},
    );
    return MessageModel.fromJson(response.data!);
  }

  @override
  Future<MessageModel> editMessage(
    String channelId,
    String messageId,
    String content,
  ) async {
    final response = await _client.patch<Map<String, dynamic>>(
      '/channels/$channelId/messages/$messageId',
      data: {'content': content},
    );
    return MessageModel.fromJson(response.data!);
  }

  @override
  Future<void> deleteMessage(String channelId, String messageId) async {
    await _client.delete('/channels/$channelId/messages/$messageId');
  }

  @override
  Future<void> addReaction(
    String channelId,
    String messageId,
    String emoji,
  ) async {
    await _client.put(
      '/channels/$channelId/messages/$messageId/reactions/$emoji/@me',
    );
  }

  @override
  Future<void> removeReaction(
    String channelId,
    String messageId,
    String emoji,
  ) async {
    await _client.delete(
      '/channels/$channelId/messages/$messageId/reactions/$emoji/@me',
    );
  }

  void dispose() {
    _messageController.close();
  }
}
