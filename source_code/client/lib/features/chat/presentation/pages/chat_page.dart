import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';

import '../../../../shared/presentation/theme/app_theme.dart';
import '../../domain/entities/message.dart';
import '../bloc/chat_bloc.dart';

class ChatPage extends StatefulWidget {
  final String channelId;
  final String channelName;

  const ChatPage({
    super.key,
    required this.channelId,
    required this.channelName,
  });

  @override
  State<ChatPage> createState() => _ChatPageState();
}

class _ChatPageState extends State<ChatPage> {
  final _messageController = TextEditingController();
  final _scrollController = ScrollController();
  final _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    context.read<ChatBloc>().add(ChatChannelOpened(widget.channelId));
    _scrollController.addListener(_onScroll);
  }

  @override
  void didUpdateWidget(ChatPage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.channelId != widget.channelId) {
      context.read<ChatBloc>().add(ChatChannelOpened(widget.channelId));
    }
  }

  @override
  void dispose() {
    _messageController.dispose();
    _scrollController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  void _onScroll() {
    final state = context.read<ChatBloc>().state;
    if (state is ChatLoaded &&
        state.hasMore &&
        _scrollController.position.pixels >=
            _scrollController.position.maxScrollExtent - 200) {
      final lastMessage = state.messages.lastOrNull;
      if (lastMessage != null) {
        context.read<ChatBloc>().add(
              ChatMessagesLoadRequested(widget.channelId, before: lastMessage.id),
            );
      }
    }
  }

  void _sendMessage() {
    final content = _messageController.text.trim();
    if (content.isEmpty) return;

    context.read<ChatBloc>().add(ChatMessageSent(content));
    _messageController.clear();
    _focusNode.requestFocus();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Channel header
        Container(
          height: 48,
          padding: const EdgeInsets.symmetric(horizontal: 16),
          decoration: BoxDecoration(
            color: AppTheme.backgroundPrimary,
            border: Border(
              bottom: BorderSide(color: Colors.black.withValues(alpha: 0.2)),
            ),
          ),
          child: Row(
            children: [
              const Icon(Icons.tag, size: 24, color: Colors.grey),
              const SizedBox(width: 8),
              Text(
                widget.channelName,
                style: const TextStyle(
                  fontWeight: FontWeight.bold,
                  fontSize: 16,
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.push_pin_outlined),
                onPressed: () {},
              ),
              IconButton(
                icon: const Icon(Icons.people_outline),
                onPressed: () {},
              ),
              IconButton(
                icon: const Icon(Icons.search),
                onPressed: () {},
              ),
            ],
          ),
        ),

        // Messages area
        Expanded(
          child: BlocBuilder<ChatBloc, ChatState>(
            builder: (context, state) {
              if (state is ChatLoading) {
                return const Center(child: CircularProgressIndicator());
              }

              if (state is ChatError) {
                return Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(state.message),
                      const SizedBox(height: 16),
                      ElevatedButton(
                        onPressed: () {
                          context.read<ChatBloc>().add(
                                ChatChannelOpened(widget.channelId),
                              );
                        },
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                );
              }

              if (state is ChatLoaded) {
                if (state.messages.isEmpty) {
                  return Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.chat_bubble_outline,
                          size: 64,
                          color: Colors.grey.shade600,
                        ),
                        const SizedBox(height: 16),
                        Text(
                          'Welcome to #${widget.channelName}!',
                          style: Theme.of(context).textTheme.headlineSmall,
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'This is the start of the channel.',
                          style: TextStyle(color: Colors.grey.shade400),
                        ),
                      ],
                    ),
                  );
                }

                return ListView.builder(
                  controller: _scrollController,
                  reverse: true,
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  itemCount: state.messages.length,
                  itemBuilder: (context, index) {
                    final message = state.messages[index];
                    final prevMessage = index < state.messages.length - 1
                        ? state.messages[index + 1]
                        : null;

                    final showHeader = prevMessage == null ||
                        prevMessage.authorId != message.authorId ||
                        message.createdAt.difference(prevMessage.createdAt).inMinutes > 5;

                    return _MessageItem(
                      message: message,
                      showHeader: showHeader,
                      onDelete: () {
                        context.read<ChatBloc>().add(ChatMessageDeleted(message.id));
                      },
                    );
                  },
                );
              }

              return const SizedBox.shrink();
            },
          ),
        ),

        // Message input
        Container(
          padding: const EdgeInsets.all(16),
          child: Row(
            children: [
              IconButton(
                icon: const Icon(Icons.add_circle_outline),
                onPressed: () {},
              ),
              Expanded(
                child: TextField(
                  controller: _messageController,
                  focusNode: _focusNode,
                  decoration: InputDecoration(
                    hintText: 'Message #${widget.channelName}',
                    filled: true,
                    fillColor: AppTheme.backgroundModifierAccent,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(8),
                      borderSide: BorderSide.none,
                    ),
                    contentPadding: const EdgeInsets.symmetric(
                      horizontal: 16,
                      vertical: 12,
                    ),
                  ),
                  onSubmitted: (_) => _sendMessage(),
                ),
              ),
              const SizedBox(width: 8),
              IconButton(
                icon: const Icon(Icons.emoji_emotions_outlined),
                onPressed: () {},
              ),
              IconButton(
                icon: const Icon(Icons.send),
                onPressed: _sendMessage,
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class _MessageItem extends StatelessWidget {
  final Message message;
  final bool showHeader;
  final VoidCallback onDelete;

  const _MessageItem({
    required this.message,
    required this.showHeader,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    final timeFormat = DateFormat.jm();
    final dateFormat = DateFormat.yMMMd();

    return Padding(
      padding: EdgeInsets.only(
        left: 16,
        right: 16,
        top: showHeader ? 16 : 2,
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (showHeader)
            CircleAvatar(
              radius: 20,
              backgroundColor: AppTheme.blurple,
              child: Text(
                message.authorUsername.substring(0, 1).toUpperCase(),
                style: const TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.bold,
                ),
              ),
            )
          else
            const SizedBox(width: 40),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (showHeader)
                  Row(
                    children: [
                      Text(
                        message.authorEffectiveName,
                        style: const TextStyle(
                          fontWeight: FontWeight.bold,
                          fontSize: 16,
                        ),
                      ),
                      const SizedBox(width: 8),
                      Text(
                        _formatTimestamp(message.createdAt, timeFormat, dateFormat),
                        style: TextStyle(
                          color: Colors.grey.shade500,
                          fontSize: 12,
                        ),
                      ),
                    ],
                  ),
                const SizedBox(height: 4),
                SelectableText(
                  message.content,
                  style: const TextStyle(fontSize: 15),
                ),
                if (message.reactions.isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 4),
                    child: Wrap(
                      spacing: 4,
                      children: message.reactions.map((r) {
                        return Container(
                          padding: const EdgeInsets.symmetric(
                            horizontal: 6,
                            vertical: 2,
                          ),
                          decoration: BoxDecoration(
                            color: r.me
                                ? AppTheme.blurple.withValues(alpha: 0.3)
                                : AppTheme.backgroundModifierAccent,
                            borderRadius: BorderRadius.circular(4),
                            border: r.me
                                ? Border.all(color: AppTheme.blurple)
                                : null,
                          ),
                          child: Text(
                            '${r.emoji} ${r.count}',
                            style: const TextStyle(fontSize: 12),
                          ),
                        );
                      }).toList(),
                    ),
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  String _formatTimestamp(
    DateTime timestamp,
    DateFormat timeFormat,
    DateFormat dateFormat,
  ) {
    final now = DateTime.now();
    final today = DateTime(now.year, now.month, now.day);
    final yesterday = today.subtract(const Duration(days: 1));
    final messageDate = DateTime(
      timestamp.year,
      timestamp.month,
      timestamp.day,
    );

    if (messageDate == today) {
      return 'Today at ${timeFormat.format(timestamp)}';
    } else if (messageDate == yesterday) {
      return 'Yesterday at ${timeFormat.format(timestamp)}';
    } else {
      return '${dateFormat.format(timestamp)} ${timeFormat.format(timestamp)}';
    }
  }
}

class EmptyChatPlaceholder extends StatelessWidget {
  const EmptyChatPlaceholder({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.forum_outlined,
            size: 64,
            color: Colors.grey.shade600,
          ),
          const SizedBox(height: 16),
          Text(
            'Select a channel',
            style: Theme.of(context).textTheme.headlineSmall?.copyWith(
                  color: Colors.grey.shade400,
                ),
          ),
          const SizedBox(height: 8),
          Text(
            'Choose a channel from the sidebar to start chatting',
            style: TextStyle(color: Colors.grey.shade500),
          ),
        ],
      ),
    );
  }
}
