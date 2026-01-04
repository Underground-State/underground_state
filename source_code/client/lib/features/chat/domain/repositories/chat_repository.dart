import 'package:dartz/dartz.dart';

import '../../../../core/error/failures.dart';
import '../entities/message.dart';

abstract class ChatRepository {
  Future<Either<Failure, List<Message>>> getMessages(
    String channelId, {
    String? before,
    int limit = 50,
  });

  Future<Either<Failure, Message>> sendMessage(
    String channelId,
    String content,
  );

  Future<Either<Failure, Message>> editMessage(
    String channelId,
    String messageId,
    String content,
  );

  Future<Either<Failure, void>> deleteMessage(
    String channelId,
    String messageId,
  );

  Future<Either<Failure, void>> addReaction(
    String channelId,
    String messageId,
    String emoji,
  );

  Future<Either<Failure, void>> removeReaction(
    String channelId,
    String messageId,
    String emoji,
  );

  Stream<Message> get messageStream;

  void connectToChannel(String channelId);
  void disconnectFromChannel(String channelId);
}
