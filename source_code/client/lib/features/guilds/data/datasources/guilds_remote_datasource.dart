import '../../../../core/network/api_client.dart';
import '../../domain/entities/guild.dart';
import '../models/guild_model.dart';

abstract class GuildsRemoteDataSource {
  Future<List<GuildModel>> getGuilds();
  Future<GuildModel> getGuild(String guildId);
  Future<GuildModel> createGuild(String name, String? description);
  Future<void> deleteGuild(String guildId);

  Future<List<ChannelModel>> getChannels(String guildId);
  Future<ChannelModel> createChannel(String guildId, String name, ChannelType type);
  Future<void> deleteChannel(String channelId);

  Future<List<GuildMemberModel>> getMembers(String guildId);
  Future<void> joinGuild(String inviteCode);
  Future<void> leaveGuild(String guildId);
}

class GuildsRemoteDataSourceImpl implements GuildsRemoteDataSource {
  final ApiClient _client;

  GuildsRemoteDataSourceImpl(this._client);

  @override
  Future<List<GuildModel>> getGuilds() async {
    final response = await _client.get<List<dynamic>>('/users/@me/guilds');
    return (response.data ?? [])
        .map((json) => GuildModel.fromJson(json as Map<String, dynamic>))
        .toList();
  }

  @override
  Future<GuildModel> getGuild(String guildId) async {
    final response = await _client.get<Map<String, dynamic>>('/guilds/$guildId');
    return GuildModel.fromJson(response.data!);
  }

  @override
  Future<GuildModel> createGuild(String name, String? description) async {
    final response = await _client.post<Map<String, dynamic>>(
      '/guilds',
      data: {
        'name': name,
        if (description != null) 'description': description,
      },
    );
    return GuildModel.fromJson(response.data!);
  }

  @override
  Future<void> deleteGuild(String guildId) async {
    await _client.delete('/guilds/$guildId');
  }

  @override
  Future<List<ChannelModel>> getChannels(String guildId) async {
    final response = await _client.get<List<dynamic>>('/guilds/$guildId/channels');
    return (response.data ?? [])
        .map((json) => ChannelModel.fromJson(json as Map<String, dynamic>))
        .toList();
  }

  @override
  Future<ChannelModel> createChannel(String guildId, String name, ChannelType type) async {
    final response = await _client.post<Map<String, dynamic>>(
      '/guilds/$guildId/channels',
      data: {
        'name': name,
        'channel_type': type.value,
      },
    );
    return ChannelModel.fromJson(response.data!);
  }

  @override
  Future<void> deleteChannel(String channelId) async {
    await _client.delete('/channels/$channelId');
  }

  @override
  Future<List<GuildMemberModel>> getMembers(String guildId) async {
    final response = await _client.get<List<dynamic>>('/guilds/$guildId/members');
    return (response.data ?? [])
        .map((json) => GuildMemberModel.fromJson(json as Map<String, dynamic>))
        .toList();
  }

  @override
  Future<void> joinGuild(String inviteCode) async {
    await _client.post('/invites/$inviteCode');
  }

  @override
  Future<void> leaveGuild(String guildId) async {
    await _client.delete('/users/@me/guilds/$guildId');
  }
}
