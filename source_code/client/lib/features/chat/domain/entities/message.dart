import 'package:equatable/equatable.dart';

class Message extends Equatable {
  final String id;
  final String channelId;
  final String authorId;
  final String authorUsername;
  final String? authorDisplayName;
  final String? authorAvatarHash;
  final String content;
  final DateTime createdAt;
  final DateTime? editedAt;
  final List<Attachment> attachments;
  final List<Reaction> reactions;

  const Message({
    required this.id,
    required this.channelId,
    required this.authorId,
    required this.authorUsername,
    this.authorDisplayName,
    this.authorAvatarHash,
    required this.content,
    required this.createdAt,
    this.editedAt,
    this.attachments = const [],
    this.reactions = const [],
  });

  String get authorEffectiveName => authorDisplayName ?? authorUsername;

  @override
  List<Object?> get props => [
        id,
        channelId,
        authorId,
        authorUsername,
        authorDisplayName,
        authorAvatarHash,
        content,
        createdAt,
        editedAt,
        attachments,
        reactions,
      ];
}

class Attachment extends Equatable {
  final String id;
  final String filename;
  final String contentType;
  final int size;
  final String url;
  final int? width;
  final int? height;

  const Attachment({
    required this.id,
    required this.filename,
    required this.contentType,
    required this.size,
    required this.url,
    this.width,
    this.height,
  });

  bool get isImage => contentType.startsWith('image/');

  @override
  List<Object?> get props => [id, filename, contentType, size, url, width, height];
}

class Reaction extends Equatable {
  final String emoji;
  final int count;
  final bool me;

  const Reaction({
    required this.emoji,
    required this.count,
    required this.me,
  });

  @override
  List<Object?> get props => [emoji, count, me];
}
