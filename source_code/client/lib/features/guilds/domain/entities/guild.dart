import 'package:equatable/equatable.dart';

class Guild extends Equatable {
  final String id;
  final String ownerId;
  final String name;
  final String? description;
  final String? iconHash;
  final int memberCount;
  final DateTime createdAt;

  const Guild({
    required this.id,
    required this.ownerId,
    required this.name,
    this.description,
    this.iconHash,
    required this.memberCount,
    required this.createdAt,
  });

  @override
  List<Object?> get props => [id, ownerId, name, description, iconHash, memberCount, createdAt];
}

class Channel extends Equatable {
  final String id;
  final String guildId;
  final String name;
  final ChannelType type;
  final int position;

  const Channel({
    required this.id,
    required this.guildId,
    required this.name,
    required this.type,
    required this.position,
  });

  @override
  List<Object?> get props => [id, guildId, name, type, position];
}

enum ChannelType {
  text(0),
  voice(1);

  final int value;
  const ChannelType(this.value);

  static ChannelType fromValue(int value) {
    return ChannelType.values.firstWhere(
      (type) => type.value == value,
      orElse: () => ChannelType.text,
    );
  }
}

class GuildMember extends Equatable {
  final String id;
  final String guildId;
  final String userId;
  final String username;
  final String? displayName;
  final String? avatarHash;
  final DateTime joinedAt;

  const GuildMember({
    required this.id,
    required this.guildId,
    required this.userId,
    required this.username,
    this.displayName,
    this.avatarHash,
    required this.joinedAt,
  });

  String get effectiveName => displayName ?? username;

  @override
  List<Object?> get props => [id, guildId, userId, username, displayName, avatarHash, joinedAt];
}
