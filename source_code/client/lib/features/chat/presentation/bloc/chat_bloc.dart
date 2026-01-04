import 'dart:async';

import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../domain/entities/message.dart';
import '../../domain/repositories/chat_repository.dart';

// Events
abstract class ChatEvent extends Equatable {
  const ChatEvent();

  @override
  List<Object?> get props => [];
}

class ChatChannelOpened extends ChatEvent {
  final String channelId;

  const ChatChannelOpened(this.channelId);

  @override
  List<Object?> get props => [channelId];
}

class ChatChannelClosed extends ChatEvent {}

class ChatMessagesLoadRequested extends ChatEvent {
  final String channelId;
  final String? before;

  const ChatMessagesLoadRequested(this.channelId, {this.before});

  @override
  List<Object?> get props => [channelId, before];
}

class ChatMessageSent extends ChatEvent {
  final String content;

  const ChatMessageSent(this.content);

  @override
  List<Object?> get props => [content];
}

class ChatMessageEdited extends ChatEvent {
  final String messageId;
  final String content;

  const ChatMessageEdited(this.messageId, this.content);

  @override
  List<Object?> get props => [messageId, content];
}

class ChatMessageDeleted extends ChatEvent {
  final String messageId;

  const ChatMessageDeleted(this.messageId);

  @override
  List<Object?> get props => [messageId];
}

class ChatMessageReceived extends ChatEvent {
  final Message message;

  const ChatMessageReceived(this.message);

  @override
  List<Object?> get props => [message];
}

class ChatReactionAdded extends ChatEvent {
  final String messageId;
  final String emoji;

  const ChatReactionAdded(this.messageId, this.emoji);

  @override
  List<Object?> get props => [messageId, emoji];
}

// States
abstract class ChatState extends Equatable {
  const ChatState();

  @override
  List<Object?> get props => [];
}

class ChatInitial extends ChatState {}

class ChatLoading extends ChatState {}

class ChatLoaded extends ChatState {
  final String channelId;
  final List<Message> messages;
  final bool hasMore;
  final bool isSending;

  const ChatLoaded({
    required this.channelId,
    required this.messages,
    this.hasMore = true,
    this.isSending = false,
  });

  ChatLoaded copyWith({
    String? channelId,
    List<Message>? messages,
    bool? hasMore,
    bool? isSending,
  }) {
    return ChatLoaded(
      channelId: channelId ?? this.channelId,
      messages: messages ?? this.messages,
      hasMore: hasMore ?? this.hasMore,
      isSending: isSending ?? this.isSending,
    );
  }

  @override
  List<Object?> get props => [channelId, messages, hasMore, isSending];
}

class ChatError extends ChatState {
  final String message;

  const ChatError(this.message);

  @override
  List<Object?> get props => [message];
}

// Bloc
class ChatBloc extends Bloc<ChatEvent, ChatState> {
  final ChatRepository _chatRepository;
  StreamSubscription<Message>? _messageSubscription;
  String? _currentChannelId;

  ChatBloc(this._chatRepository) : super(ChatInitial()) {
    on<ChatChannelOpened>(_onChannelOpened);
    on<ChatChannelClosed>(_onChannelClosed);
    on<ChatMessagesLoadRequested>(_onMessagesLoadRequested);
    on<ChatMessageSent>(_onMessageSent);
    on<ChatMessageEdited>(_onMessageEdited);
    on<ChatMessageDeleted>(_onMessageDeleted);
    on<ChatMessageReceived>(_onMessageReceived);
    on<ChatReactionAdded>(_onReactionAdded);
  }

  Future<void> _onChannelOpened(
    ChatChannelOpened event,
    Emitter<ChatState> emit,
  ) async {
    // Unsubscribe from previous channel
    if (_currentChannelId != null) {
      _chatRepository.disconnectFromChannel(_currentChannelId!);
    }

    _currentChannelId = event.channelId;
    _chatRepository.connectToChannel(event.channelId);

    // Subscribe to message stream
    _messageSubscription?.cancel();
    _messageSubscription = _chatRepository.messageStream.listen((message) {
      if (message.channelId == _currentChannelId) {
        add(ChatMessageReceived(message));
      }
    });

    // Load messages
    add(ChatMessagesLoadRequested(event.channelId));
  }

  Future<void> _onChannelClosed(
    ChatChannelClosed event,
    Emitter<ChatState> emit,
  ) async {
    if (_currentChannelId != null) {
      _chatRepository.disconnectFromChannel(_currentChannelId!);
      _currentChannelId = null;
    }
    _messageSubscription?.cancel();
    _messageSubscription = null;
    emit(ChatInitial());
  }

  Future<void> _onMessagesLoadRequested(
    ChatMessagesLoadRequested event,
    Emitter<ChatState> emit,
  ) async {
    final currentState = state;
    final isLoadingMore = event.before != null && currentState is ChatLoaded;

    if (!isLoadingMore) {
      emit(ChatLoading());
    }

    final result = await _chatRepository.getMessages(
      event.channelId,
      before: event.before,
    );

    result.fold(
      (failure) => emit(ChatError(failure.message)),
      (messages) {
        if (isLoadingMore && currentState is ChatLoaded) {
          emit(currentState.copyWith(
            messages: [...currentState.messages, ...messages],
            hasMore: messages.length >= 50,
          ));
        } else {
          emit(ChatLoaded(
            channelId: event.channelId,
            messages: messages,
            hasMore: messages.length >= 50,
          ));
        }
      },
    );
  }

  Future<void> _onMessageSent(
    ChatMessageSent event,
    Emitter<ChatState> emit,
  ) async {
    final currentState = state;
    if (currentState is! ChatLoaded) return;

    emit(currentState.copyWith(isSending: true));

    final result = await _chatRepository.sendMessage(
      currentState.channelId,
      event.content,
    );

    result.fold(
      (failure) => emit(currentState.copyWith(isSending: false)),
      (message) {
        emit(currentState.copyWith(
          messages: [message, ...currentState.messages],
          isSending: false,
        ));
      },
    );
  }

  Future<void> _onMessageEdited(
    ChatMessageEdited event,
    Emitter<ChatState> emit,
  ) async {
    final currentState = state;
    if (currentState is! ChatLoaded) return;

    final result = await _chatRepository.editMessage(
      currentState.channelId,
      event.messageId,
      event.content,
    );

    result.fold(
      (failure) => {},
      (message) {
        final updatedMessages = currentState.messages.map((m) {
          return m.id == event.messageId ? message : m;
        }).toList();
        emit(currentState.copyWith(messages: updatedMessages));
      },
    );
  }

  Future<void> _onMessageDeleted(
    ChatMessageDeleted event,
    Emitter<ChatState> emit,
  ) async {
    final currentState = state;
    if (currentState is! ChatLoaded) return;

    final result = await _chatRepository.deleteMessage(
      currentState.channelId,
      event.messageId,
    );

    result.fold(
      (failure) => {},
      (_) {
        final updatedMessages = currentState.messages
            .where((m) => m.id != event.messageId)
            .toList();
        emit(currentState.copyWith(messages: updatedMessages));
      },
    );
  }

  Future<void> _onMessageReceived(
    ChatMessageReceived event,
    Emitter<ChatState> emit,
  ) async {
    final currentState = state;
    if (currentState is! ChatLoaded) return;

    // Check if message already exists (could be our own sent message)
    final exists = currentState.messages.any((m) => m.id == event.message.id);
    if (exists) {
      // Update existing message
      final updatedMessages = currentState.messages.map((m) {
        return m.id == event.message.id ? event.message : m;
      }).toList();
      emit(currentState.copyWith(messages: updatedMessages));
    } else {
      // Add new message at the beginning
      emit(currentState.copyWith(
        messages: [event.message, ...currentState.messages],
      ));
    }
  }

  Future<void> _onReactionAdded(
    ChatReactionAdded event,
    Emitter<ChatState> emit,
  ) async {
    final currentState = state;
    if (currentState is! ChatLoaded) return;

    await _chatRepository.addReaction(
      currentState.channelId,
      event.messageId,
      event.emoji,
    );
  }

  @override
  Future<void> close() {
    _messageSubscription?.cancel();
    if (_currentChannelId != null) {
      _chatRepository.disconnectFromChannel(_currentChannelId!);
    }
    return super.close();
  }
}
