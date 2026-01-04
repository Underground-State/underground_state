import '../../domain/entities/user.dart';

class UserModel extends User {
  const UserModel({
    required super.id,
    required super.walletAddress,
    required super.username,
    super.displayName,
    super.avatarHash,
    required super.kycStatus,
  });

  factory UserModel.fromJson(Map<String, dynamic> json) {
    return UserModel(
      id: json['id'] as String,
      walletAddress: json['wallet_address'] as String,
      username: json['username'] as String,
      displayName: json['display_name'] as String?,
      avatarHash: json['avatar_hash'] as String?,
      kycStatus: json['kyc_status'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'wallet_address': walletAddress,
      'username': username,
      'display_name': displayName,
      'avatar_hash': avatarHash,
      'kyc_status': kycStatus,
    };
  }
}
