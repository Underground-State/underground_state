import 'package:dartz/dartz.dart';

import '../../../../core/error/failures.dart';
import '../entities/user.dart';

abstract class AuthRepository {
  Future<Either<Failure, String>> getNonce(String walletAddress);
  Future<Either<Failure, User>> verifySignature({
    required String walletAddress,
    required String signature,
    required String message,
    required String nonce,
    required String hcaptchaToken,
  });
  Future<Either<Failure, User>> refreshToken();
  Future<void> logout();
  Future<bool> isLoggedIn();
}
