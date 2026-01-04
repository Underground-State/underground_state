import 'secure_storage.dart';

class SessionManager {
  final SecureStorageService _storage;

  static const _accessTokenKey = 'access_token';
  static const _refreshTokenKey = 'refresh_token';
  static const _walletAddressKey = 'wallet_address';
  static const _userIdKey = 'user_id';

  SessionManager(this._storage);

  Future<void> saveSession({
    required String accessToken,
    required String refreshToken,
    required String walletAddress,
    required String userId,
  }) async {
    await _storage.write(_accessTokenKey, accessToken);
    await _storage.write(_refreshTokenKey, refreshToken);
    await _storage.write(_walletAddressKey, walletAddress);
    await _storage.write(_userIdKey, userId);
  }

  Future<String?> getAccessToken() async {
    return _storage.read(_accessTokenKey);
  }

  Future<String?> getRefreshToken() async {
    return _storage.read(_refreshTokenKey);
  }

  Future<String?> getWalletAddress() async {
    return _storage.read(_walletAddressKey);
  }

  Future<String?> getUserId() async {
    return _storage.read(_userIdKey);
  }

  Future<bool> isLoggedIn() async {
    final token = await getAccessToken();
    return token != null;
  }

  Future<void> clearSession() async {
    await _storage.deleteAll();
  }
}
