import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';

import '../../../../shared/presentation/theme/app_theme.dart';
import '../bloc/auth_bloc.dart';

class LoginPage extends StatelessWidget {
  const LoginPage({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocListener<AuthBloc, AuthState>(
      listener: (context, state) {
        if (state is AuthAuthenticated) {
          context.go('/guilds');
        } else if (state is AuthError) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(state.message),
              backgroundColor: AppTheme.red,
            ),
          );
        }
      },
      child: Scaffold(
        body: Center(
          child: Container(
            constraints: const BoxConstraints(maxWidth: 400),
            padding: const EdgeInsets.all(32),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                // Logo
                Container(
                  width: 80,
                  height: 80,
                  decoration: BoxDecoration(
                    color: AppTheme.blurple,
                    borderRadius: BorderRadius.circular(20),
                  ),
                  child: const Icon(
                    Icons.discord,
                    size: 48,
                    color: Colors.white,
                  ),
                ),
                const SizedBox(height: 24),
                Text(
                  'Underground State',
                  style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                        fontWeight: FontWeight.bold,
                      ),
                ),
                const SizedBox(height: 8),
                Text(
                  'Connect your wallet to continue',
                  style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                        color: Colors.grey,
                      ),
                ),
                const SizedBox(height: 48),

                // Wallet buttons
                _WalletButton(
                  icon: Icons.account_balance_wallet,
                  label: 'MetaMask',
                  onPressed: () => _connectWallet(context, 'metamask'),
                ),
                const SizedBox(height: 12),
                _WalletButton(
                  icon: Icons.water_drop,
                  label: 'Phantom',
                  onPressed: () => _connectWallet(context, 'phantom'),
                ),
                const SizedBox(height: 12),
                _WalletButton(
                  icon: Icons.waves,
                  label: 'SUI Wallet',
                  onPressed: () => _connectWallet(context, 'sui'),
                ),

                const SizedBox(height: 32),
                Text(
                  'By connecting, you agree to our Terms of Service',
                  style: Theme.of(context).textTheme.bodySmall?.copyWith(
                        color: Colors.grey,
                      ),
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  void _connectWallet(BuildContext context, String walletType) {
    // In production, this would use flutter_web3 to connect
    // For now, show a demo login
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: const Text('Demo Mode'),
        content: const Text(
          'In production, this would connect to your wallet. '
          'For demo, click Continue to simulate login.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.pop(ctx);
              // Simulate login
              context.read<AuthBloc>().add(const AuthLoginRequested(
                    walletAddress: '0x1234567890abcdef',
                    signature: '0xsignature',
                    message: 'Sign in message',
                    nonce: 'nonce123',
                  ));
            },
            child: const Text('Continue'),
          ),
        ],
      ),
    );
  }
}

class _WalletButton extends StatelessWidget {
  final IconData icon;
  final String label;
  final VoidCallback onPressed;

  const _WalletButton({
    required this.icon,
    required this.label,
    required this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: OutlinedButton.icon(
        onPressed: onPressed,
        icon: Icon(icon),
        label: Text(label),
        style: OutlinedButton.styleFrom(
          padding: const EdgeInsets.symmetric(vertical: 16),
          side: BorderSide(color: Colors.grey.shade700),
        ),
      ),
    );
  }
}
