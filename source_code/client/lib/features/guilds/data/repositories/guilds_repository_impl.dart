import 'package:dartz/dartz.dart';

import '../../../../core/error/failures.dart';
import '../../domain/entities/guild.dart';
import '../../domain/repositories/guilds_repository.dart';
import '../datasources/guilds_remote_datasource.dart';

class GuildsRepositoryImpl implements GuildsRepository {
  final GuildsRemoteDataSource _remoteDataSource;

  GuildsRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, List<Guild>>> getGuilds() async {
    try {
      final guilds = await _remoteDataSource.getGuilds();
      return Right(guilds);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, Guild>> getGuild(String guildId) async {
    try {
      final guild = await _remoteDataSource.getGuild(guildId);
      return Right(guild);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, Guild>> createGuild(String name, String? description) async {
    try {
      final guild = await _remoteDataSource.createGuild(name, description);
      return Right(guild);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, void>> deleteGuild(String guildId) async {
    try {
      await _remoteDataSource.deleteGuild(guildId);
      return const Right(null);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, List<Channel>>> getChannels(String guildId) async {
    try {
      final channels = await _remoteDataSource.getChannels(guildId);
      return Right(channels);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, Channel>> createChannel(
    String guildId,
    String name,
    ChannelType type,
  ) async {
    try {
      final channel = await _remoteDataSource.createChannel(guildId, name, type);
      return Right(channel);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, void>> deleteChannel(String channelId) async {
    try {
      await _remoteDataSource.deleteChannel(channelId);
      return const Right(null);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, List<GuildMember>>> getMembers(String guildId) async {
    try {
      final members = await _remoteDataSource.getMembers(guildId);
      return Right(members);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, void>> joinGuild(String inviteCode) async {
    try {
      await _remoteDataSource.joinGuild(inviteCode);
      return const Right(null);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, void>> leaveGuild(String guildId) async {
    try {
      await _remoteDataSource.leaveGuild(guildId);
      return const Right(null);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }
}
