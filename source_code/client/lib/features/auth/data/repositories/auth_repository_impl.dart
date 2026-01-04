import 'package:dartz/dartz.dart';

import '../../../../core/error/failures.dart';
import '../../../../core/storage/session_manager.dart';
import '../../domain/entities/user.dart';
import '../../domain/repositories/auth_repository.dart';
import '../datasources/auth_remote_datasource.dart';

class AuthRepositoryImpl implements AuthRepository {
  final AuthRemoteDataSource _remoteDataSource;
  final SessionManager _sessionManager;

  AuthRepositoryImpl(this._remoteDataSource, this._sessionManager);

  @override
  Future<Either<Failure, String>> getNonce(String walletAddress) async {
    try {
      final response = await _remoteDataSource.getNonce(walletAddress);
      return Right(response.message);
    } catch (e) {
      return const Left(ServerFailure());
    }
  }

  @override
  Future<Either<Failure, User>> verifySignature({
    required String walletAddress,
    required String signature,
    required String message,
    required String nonce,
    required String hcaptchaToken,
  }) async {
    try {
      final response = await _remoteDataSource.verifySignature(
        walletAddress: walletAddress,
        signature: signature,
        message: message,
        nonce: nonce,
        hcaptchaToken: hcaptchaToken,
      );

      await _sessionManager.saveSession(
        accessToken: response.accessToken,
        refreshToken: response.refreshToken,
        walletAddress: response.user.walletAddress,
        userId: response.user.id,
      );

      return Right(response.user);
    } catch (e) {
      return const Left(AuthFailure());
    }
  }

  @override
  Future<Either<Failure, User>> refreshToken() async {
    try {
      final refreshToken = await _sessionManager.getRefreshToken();
      if (refreshToken == null) {
        return const Left(AuthFailure('No refresh token'));
      }

      final response = await _remoteDataSource.refreshToken(refreshToken);

      await _sessionManager.saveSession(
        accessToken: response.accessToken,
        refreshToken: response.refreshToken,
        walletAddress: response.user.walletAddress,
        userId: response.user.id,
      );

      return Right(response.user);
    } catch (e) {
      return const Left(AuthFailure());
    }
  }

  @override
  Future<void> logout() async {
    await _sessionManager.clearSession();
  }

  @override
  Future<bool> isLoggedIn() async {
    return _sessionManager.isLoggedIn();
  }
}
