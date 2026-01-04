import '../../domain/entities/message.dart';

class MessageModel extends Message {
  const MessageModel({
    required super.id,
    required super.channelId,
    required super.authorId,
    required super.authorUsername,
    super.authorDisplayName,
    super.authorAvatarHash,
    required super.content,
    required super.createdAt,
    super.editedAt,
    super.attachments = const [],
    super.reactions = const [],
  });

  factory MessageModel.fromJson(Map<String, dynamic> json) {
    return MessageModel(
      id: json['id'] as String,
      channelId: json['channel_id'] as String,
      authorId: json['author']['id'] as String,
      authorUsername: json['author']['username'] as String,
      authorDisplayName: json['author']['display_name'] as String?,
      authorAvatarHash: json['author']['avatar_hash'] as String?,
      content: json['content'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      editedAt: json['edited_at'] != null
          ? DateTime.parse(json['edited_at'] as String)
          : null,
      attachments: (json['attachments'] as List<dynamic>?)
              ?.map((a) => AttachmentModel.fromJson(a as Map<String, dynamic>))
              .toList() ??
          [],
      reactions: (json['reactions'] as List<dynamic>?)
              ?.map((r) => ReactionModel.fromJson(r as Map<String, dynamic>))
              .toList() ??
          [],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'channel_id': channelId,
      'author': {
        'id': authorId,
        'username': authorUsername,
        'display_name': authorDisplayName,
        'avatar_hash': authorAvatarHash,
      },
      'content': content,
      'created_at': createdAt.toIso8601String(),
      'edited_at': editedAt?.toIso8601String(),
      'attachments': attachments
          .map((a) => (a as AttachmentModel).toJson())
          .toList(),
      'reactions': reactions
          .map((r) => (r as ReactionModel).toJson())
          .toList(),
    };
  }
}

class AttachmentModel extends Attachment {
  const AttachmentModel({
    required super.id,
    required super.filename,
    required super.contentType,
    required super.size,
    required super.url,
    super.width,
    super.height,
  });

  factory AttachmentModel.fromJson(Map<String, dynamic> json) {
    return AttachmentModel(
      id: json['id'] as String,
      filename: json['filename'] as String,
      contentType: json['content_type'] as String,
      size: json['size'] as int,
      url: json['url'] as String,
      width: json['width'] as int?,
      height: json['height'] as int?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'filename': filename,
      'content_type': contentType,
      'size': size,
      'url': url,
      'width': width,
      'height': height,
    };
  }
}

class ReactionModel extends Reaction {
  const ReactionModel({
    required super.emoji,
    required super.count,
    required super.me,
  });

  factory ReactionModel.fromJson(Map<String, dynamic> json) {
    return ReactionModel(
      emoji: json['emoji'] as String,
      count: json['count'] as int,
      me: json['me'] as bool? ?? false,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'emoji': emoji,
      'count': count,
      'me': me,
    };
  }
}
