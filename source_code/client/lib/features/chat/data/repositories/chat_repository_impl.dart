import 'package:dartz/dartz.dart';

import '../../../../core/error/failures.dart';
import '../../domain/entities/message.dart';
import '../../domain/repositories/chat_repository.dart';
import '../datasources/chat_remote_datasource.dart';

class ChatRepositoryImpl implements ChatRepository {
  final ChatRemoteDataSource _remoteDataSource;

  ChatRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, List<Message>>> getMessages(
    String channelId, {
    String? before,
    int limit = 50,
  }) async {
    try {
      final messages = await _remoteDataSource.getMessages(
        channelId,
        before: before,
        limit: limit,
      );
      return Right(messages);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, Message>> sendMessage(
    String channelId,
    String content,
  ) async {
    try {
      final message = await _remoteDataSource.sendMessage(channelId, content);
      return Right(message);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, Message>> editMessage(
    String channelId,
    String messageId,
    String content,
  ) async {
    try {
      final message = await _remoteDataSource.editMessage(
        channelId,
        messageId,
        content,
      );
      return Right(message);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, void>> deleteMessage(
    String channelId,
    String messageId,
  ) async {
    try {
      await _remoteDataSource.deleteMessage(channelId, messageId);
      return const Right(null);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, void>> addReaction(
    String channelId,
    String messageId,
    String emoji,
  ) async {
    try {
      await _remoteDataSource.addReaction(channelId, messageId, emoji);
      return const Right(null);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, void>> removeReaction(
    String channelId,
    String messageId,
    String emoji,
  ) async {
    try {
      await _remoteDataSource.removeReaction(channelId, messageId, emoji);
      return const Right(null);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Stream<Message> get messageStream => _remoteDataSource.messageStream;

  @override
  void connectToChannel(String channelId) {
    _remoteDataSource.subscribeToChannel(channelId);
  }

  @override
  void disconnectFromChannel(String channelId) {
    _remoteDataSource.unsubscribeFromChannel(channelId);
  }
}
