import '../../domain/entities/guild.dart';

class GuildModel extends Guild {
  const GuildModel({
    required super.id,
    required super.ownerId,
    required super.name,
    super.description,
    super.iconHash,
    required super.memberCount,
    required super.createdAt,
  });

  factory GuildModel.fromJson(Map<String, dynamic> json) {
    return GuildModel(
      id: json['id'] as String,
      ownerId: json['owner_id'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      iconHash: json['icon_hash'] as String?,
      memberCount: json['member_count'] as int? ?? 0,
      createdAt: DateTime.parse(json['created_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'owner_id': ownerId,
      'name': name,
      'description': description,
      'icon_hash': iconHash,
      'member_count': memberCount,
      'created_at': createdAt.toIso8601String(),
    };
  }
}

class ChannelModel extends Channel {
  const ChannelModel({
    required super.id,
    required super.guildId,
    required super.name,
    required super.type,
    required super.position,
  });

  factory ChannelModel.fromJson(Map<String, dynamic> json) {
    return ChannelModel(
      id: json['id'] as String,
      guildId: json['guild_id'] as String,
      name: json['name'] as String,
      type: ChannelType.fromValue(json['channel_type'] as int),
      position: json['position'] as int? ?? 0,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'guild_id': guildId,
      'name': name,
      'channel_type': type.value,
      'position': position,
    };
  }
}

class GuildMemberModel extends GuildMember {
  const GuildMemberModel({
    required super.id,
    required super.guildId,
    required super.userId,
    required super.username,
    super.displayName,
    super.avatarHash,
    required super.joinedAt,
  });

  factory GuildMemberModel.fromJson(Map<String, dynamic> json) {
    return GuildMemberModel(
      id: json['id'] as String,
      guildId: json['guild_id'] as String,
      userId: json['user_id'] as String,
      username: json['username'] as String,
      displayName: json['display_name'] as String?,
      avatarHash: json['avatar_hash'] as String?,
      joinedAt: DateTime.parse(json['joined_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'guild_id': guildId,
      'user_id': userId,
      'username': username,
      'display_name': displayName,
      'avatar_hash': avatarHash,
      'joined_at': joinedAt.toIso8601String(),
    };
  }
}
