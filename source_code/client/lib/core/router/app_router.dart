import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';

import '../../features/auth/presentation/pages/login_page.dart';
import '../../features/chat/presentation/bloc/chat_bloc.dart';
import '../../features/chat/presentation/pages/chat_page.dart';
import '../../features/guilds/presentation/bloc/guilds_bloc.dart';
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
      return '/guilds';
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
      builder: (context, state, child) {
        return MultiBlocProvider(
          providers: [
            BlocProvider(create: (_) => getIt<GuildsBloc>()),
            BlocProvider(create: (_) => getIt<ChatBloc>()),
          ],
          child: GuildsPage(child: child),
        );
      },
      routes: [
        GoRoute(
          path: '/guilds',
          name: 'guilds',
          builder: (context, state) => const EmptyChatPlaceholder(),
        ),
        GoRoute(
          path: '/guilds/:guildId',
          name: 'guild',
          builder: (context, state) => const EmptyChatPlaceholder(),
        ),
        GoRoute(
          path: '/guilds/:guildId/channels/:channelId',
          name: 'channel',
          builder: (context, state) {
            final channelId = state.pathParameters['channelId']!;
            // Get channel name from bloc state
            final guildsState = context.read<GuildsBloc>().state;
            String channelName = 'general';
            if (guildsState is GuildsLoaded) {
              final channel = guildsState.channels
                  .where((c) => c.id == channelId)
                  .firstOrNull;
              if (channel != null) {
                channelName = channel.name;
              }
            }
            return ChatPage(
              channelId: channelId,
              channelName: channelName,
            );
          },
        ),
      ],
    ),
  ],
);
