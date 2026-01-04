import 'package:dartz/dartz.dart';

import '../../../../core/error/failures.dart';
import '../entities/guild.dart';

abstract class GuildsRepository {
  Future<Either<Failure, List<Guild>>> getGuilds();
  Future<Either<Failure, Guild>> getGuild(String guildId);
  Future<Either<Failure, Guild>> createGuild(String name, String? description);
  Future<Either<Failure, void>> deleteGuild(String guildId);

  Future<Either<Failure, List<Channel>>> getChannels(String guildId);
  Future<Either<Failure, Channel>> createChannel(String guildId, String name, ChannelType type);
  Future<Either<Failure, void>> deleteChannel(String channelId);

  Future<Either<Failure, List<GuildMember>>> getMembers(String guildId);
  Future<Either<Failure, void>> joinGuild(String inviteCode);
  Future<Either<Failure, void>> leaveGuild(String guildId);
}
