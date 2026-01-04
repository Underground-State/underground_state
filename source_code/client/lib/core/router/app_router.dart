import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../../features/auth/presentation/pages/login_page.dart';
import '../../features/chat/presentation/pages/chat_page.dart';
import '../../features/guilds/presentation/pages/guilds_page.dart';
import '../di/injection.dart';
import '../storage/session_manager.dart';

final appRouter = GoRouter(
  initialLocation: '/login',
  redirect: (context, state) async {
    final session = getIt<SessionManager>();
    final isLoggedIn = await session.isLoggedIn();
    final isLoginRoute = state.matchedLocation == '/login';

    if (!isLoggedIn && !isLoginRoute) {
      return '/login';
    }

    if (isLoggedIn && isLoginRoute) {
      return '/';
    }

    return null;
  },
  routes: [
    GoRoute(
      path: '/login',
      name: 'login',
      builder: (context, state) => const LoginPage(),
    ),
    ShellRoute(
      builder: (context, state, child) => AppShell(child: child),
      routes: [
        GoRoute(
          path: '/',
          name: 'home',
          redirect: (_, __) => '/guilds',
        ),
        GoRoute(
          path: '/guilds',
          name: 'guilds',
          builder: (context, state) => const GuildsPage(),
        ),
        GoRoute(
          path: '/guilds/:guildId/channels/:channelId',
          name: 'channel',
          builder: (context, state) => ChatPage(
            guildId: state.pathParameters['guildId']!,
            channelId: state.pathParameters['channelId']!,
          ),
        ),
      ],
    ),
  ],
);

class AppShell extends StatelessWidget {
  final Widget child;

  const AppShell({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: [
          // Server sidebar
          Container(
            width: 72,
            color: const Color(0xFF1E1F22),
            child: const Column(
              children: [
                SizedBox(height: 12),
                // Guild icons will go here
              ],
            ),
          ),
          // Main content
          Expanded(child: child),
        ],
      ),
    );
  }
}
