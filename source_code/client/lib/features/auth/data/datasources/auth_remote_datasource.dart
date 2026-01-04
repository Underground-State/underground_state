import '../../../../core/network/api_client.dart';
import '../models/user_model.dart';

abstract class AuthRemoteDataSource {
  Future<NonceResponse> getNonce(String walletAddress);
  Future<AuthResponse> verifySignature({
    required String walletAddress,
    required String signature,
    required String message,
    required String nonce,
    required String hcaptchaToken,
  });
  Future<AuthResponse> refreshToken(String refreshToken);
}

class NonceResponse {
  final String nonce;
  final String message;
  final String expiresAt;

  NonceResponse({
    required this.nonce,
    required this.message,
    required this.expiresAt,
  });

  factory NonceResponse.fromJson(Map<String, dynamic> json) {
    return NonceResponse(
      nonce: json['nonce'] as String,
      message: json['message'] as String,
      expiresAt: json['expires_at'] as String,
    );
  }
}

class AuthResponse {
  final String accessToken;
  final String refreshToken;
  final int expiresIn;
  final UserModel user;

  AuthResponse({
    required this.accessToken,
    required this.refreshToken,
    required this.expiresIn,
    required this.user,
  });

  factory AuthResponse.fromJson(Map<String, dynamic> json) {
    return AuthResponse(
      accessToken: json['access_token'] as String,
      refreshToken: json['refresh_token'] as String,
      expiresIn: json['expires_in'] as int,
      user: UserModel.fromJson(json['user'] as Map<String, dynamic>),
    );
  }
}

class AuthRemoteDataSourceImpl implements AuthRemoteDataSource {
  final ApiClient _client;

  AuthRemoteDataSourceImpl(this._client);

  @override
  Future<NonceResponse> getNonce(String walletAddress) async {
    final response = await _client.post<Map<String, dynamic>>(
      '/auth/nonce',
      data: {'wallet_address': walletAddress},
    );
    return NonceResponse.fromJson(response.data!);
  }

  @override
  Future<AuthResponse> verifySignature({
    required String walletAddress,
    required String signature,
    required String message,
    required String nonce,
    required String hcaptchaToken,
  }) async {
    final response = await _client.post<Map<String, dynamic>>(
      '/auth/verify',
      data: {
        'wallet_address': walletAddress,
        'signature': signature,
        'message': message,
        'nonce': nonce,
        'hcaptcha_token': hcaptchaToken,
      },
    );
    return AuthResponse.fromJson(response.data!);
  }

  @override
  Future<AuthResponse> refreshToken(String refreshToken) async {
    final response = await _client.post<Map<String, dynamic>>(
      '/auth/refresh',
      data: {'refresh_token': refreshToken},
    );
    return AuthResponse.fromJson(response.data!);
  }
}
