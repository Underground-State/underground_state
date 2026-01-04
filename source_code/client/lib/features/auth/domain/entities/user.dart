import 'package:equatable/equatable.dart';

class User extends Equatable {
  final String id;
  final String walletAddress;
  final String username;
  final String? displayName;
  final String? avatarHash;
  final String kycStatus;

  const User({
    required this.id,
    required this.walletAddress,
    required this.username,
    this.displayName,
    this.avatarHash,
    required this.kycStatus,
  });

  @override
  List<Object?> get props => [
        id,
        walletAddress,
        username,
        displayName,
        avatarHash,
        kycStatus,
      ];
}
