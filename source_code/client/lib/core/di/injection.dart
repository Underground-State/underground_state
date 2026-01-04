import 'package:dio/dio.dart';
import 'package:get_it/get_it.dart';

import '../config/app_config.dart';
import '../network/api_client.dart';
import '../network/websocket_client.dart';
import '../storage/secure_storage.dart';
import '../storage/session_manager.dart';
import '../../features/auth/data/datasources/auth_remote_datasource.dart';
import '../../features/auth/data/repositories/auth_repository_impl.dart';
import '../../features/auth/domain/repositories/auth_repository.dart';
import '../../features/auth/presentation/bloc/auth_bloc.dart';
import '../../features/chat/data/datasources/chat_remote_datasource.dart';
import '../../features/chat/data/repositories/chat_repository_impl.dart';
import '../../features/chat/domain/repositories/chat_repository.dart';
import '../../features/chat/presentation/bloc/chat_bloc.dart';
import '../../features/guilds/data/datasources/guilds_remote_datasource.dart';
import '../../features/guilds/data/repositories/guilds_repository_impl.dart';
import '../../features/guilds/domain/repositories/guilds_repository.dart';
import '../../features/guilds/presentation/bloc/guilds_bloc.dart';

final getIt = GetIt.instance;

Future<void> initDependencies() async {
  // Core
  getIt.registerLazySingleton<SecureStorageService>(() => SecureStorageService());
  getIt.registerLazySingleton<SessionManager>(() => SessionManager(getIt()));

  // Network
  getIt.registerLazySingleton<Dio>(() {
    final dio = Dio(BaseOptions(
      baseUrl: AppConfig.apiBaseUrl,
      connectTimeout: const Duration(seconds: 10),
      receiveTimeout: const Duration(seconds: 10),
    ));

    dio.interceptors.add(InterceptorsWrapper(
      onRequest: (options, handler) async {
        final token = await getIt<SessionManager>().getAccessToken();
        if (token != null) {
          options.headers['Authorization'] = 'Bearer $token';
        }
        handler.next(options);
      },
    ));

    return dio;
  });

  getIt.registerLazySingleton<ApiClient>(() => ApiClient(getIt()));
  getIt.registerLazySingleton<WebSocketClient>(() => WebSocketClient());

  // Auth
  getIt.registerLazySingleton<AuthRemoteDataSource>(
    () => AuthRemoteDataSourceImpl(getIt()),
  );
  getIt.registerLazySingleton<AuthRepository>(
    () => AuthRepositoryImpl(getIt(), getIt()),
  );
  getIt.registerFactory(() => AuthBloc(getIt(), getIt()));

  // Guilds
  getIt.registerLazySingleton<GuildsRemoteDataSource>(
    () => GuildsRemoteDataSourceImpl(getIt()),
  );
  getIt.registerLazySingleton<GuildsRepository>(
    () => GuildsRepositoryImpl(getIt()),
  );
  getIt.registerFactory(() => GuildsBloc(getIt()));

  // Chat
  getIt.registerLazySingleton<ChatRemoteDataSource>(
    () => ChatRemoteDataSourceImpl(getIt(), getIt()),
  );
  getIt.registerLazySingleton<ChatRepository>(
    () => ChatRepositoryImpl(getIt()),
  );
  getIt.registerFactory(() => ChatBloc(getIt()));
}
