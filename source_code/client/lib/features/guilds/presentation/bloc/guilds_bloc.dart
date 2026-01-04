import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../domain/entities/guild.dart';
import '../../domain/repositories/guilds_repository.dart';

// Events
abstract class GuildsEvent extends Equatable {
  const GuildsEvent();

  @override
  List<Object?> get props => [];
}

class GuildsLoadRequested extends GuildsEvent {}

class GuildSelected extends GuildsEvent {
  final String guildId;

  const GuildSelected(this.guildId);

  @override
  List<Object?> get props => [guildId];
}

class GuildCreateRequested extends GuildsEvent {
  final String name;
  final String? description;

  const GuildCreateRequested(this.name, this.description);

  @override
  List<Object?> get props => [name, description];
}

class ChannelsLoadRequested extends GuildsEvent {
  final String guildId;

  const ChannelsLoadRequested(this.guildId);

  @override
  List<Object?> get props => [guildId];
}

class ChannelSelected extends GuildsEvent {
  final String channelId;

  const ChannelSelected(this.channelId);

  @override
  List<Object?> get props => [channelId];
}

class ChannelCreateRequested extends GuildsEvent {
  final String guildId;
  final String name;
  final ChannelType type;

  const ChannelCreateRequested(this.guildId, this.name, this.type);

  @override
  List<Object?> get props => [guildId, name, type];
}

// States
abstract class GuildsState extends Equatable {
  const GuildsState();

  @override
  List<Object?> get props => [];
}

class GuildsInitial extends GuildsState {}

class GuildsLoading extends GuildsState {}

class GuildsLoaded extends GuildsState {
  final List<Guild> guilds;
  final Guild? selectedGuild;
  final List<Channel> channels;
  final Channel? selectedChannel;

  const GuildsLoaded({
    required this.guilds,
    this.selectedGuild,
    this.channels = const [],
    this.selectedChannel,
  });

  GuildsLoaded copyWith({
    List<Guild>? guilds,
    Guild? selectedGuild,
    List<Channel>? channels,
    Channel? selectedChannel,
    bool clearSelectedGuild = false,
    bool clearSelectedChannel = false,
  }) {
    return GuildsLoaded(
      guilds: guilds ?? this.guilds,
      selectedGuild: clearSelectedGuild ? null : (selectedGuild ?? this.selectedGuild),
      channels: channels ?? this.channels,
      selectedChannel: clearSelectedChannel ? null : (selectedChannel ?? this.selectedChannel),
    );
  }

  @override
  List<Object?> get props => [guilds, selectedGuild, channels, selectedChannel];
}

class GuildsError extends GuildsState {
  final String message;

  const GuildsError(this.message);

  @override
  List<Object?> get props => [message];
}

// Bloc
class GuildsBloc extends Bloc<GuildsEvent, GuildsState> {
  final GuildsRepository _guildsRepository;

  GuildsBloc(this._guildsRepository) : super(GuildsInitial()) {
    on<GuildsLoadRequested>(_onLoadRequested);
    on<GuildSelected>(_onGuildSelected);
    on<GuildCreateRequested>(_onGuildCreateRequested);
    on<ChannelsLoadRequested>(_onChannelsLoadRequested);
    on<ChannelSelected>(_onChannelSelected);
    on<ChannelCreateRequested>(_onChannelCreateRequested);
  }

  Future<void> _onLoadRequested(
    GuildsLoadRequested event,
    Emitter<GuildsState> emit,
  ) async {
    emit(GuildsLoading());

    final result = await _guildsRepository.getGuilds();
    result.fold(
      (failure) => emit(GuildsError(failure.message)),
      (guilds) => emit(GuildsLoaded(guilds: guilds)),
    );
  }

  Future<void> _onGuildSelected(
    GuildSelected event,
    Emitter<GuildsState> emit,
  ) async {
    final currentState = state;
    if (currentState is! GuildsLoaded) return;

    final guild = currentState.guilds.firstWhere(
      (g) => g.id == event.guildId,
      orElse: () => currentState.guilds.first,
    );

    emit(currentState.copyWith(
      selectedGuild: guild,
      clearSelectedChannel: true,
    ));

    // Load channels for selected guild
    add(ChannelsLoadRequested(event.guildId));
  }

  Future<void> _onGuildCreateRequested(
    GuildCreateRequested event,
    Emitter<GuildsState> emit,
  ) async {
    final result = await _guildsRepository.createGuild(event.name, event.description);
    result.fold(
      (failure) => emit(GuildsError(failure.message)),
      (guild) {
        final currentState = state;
        if (currentState is GuildsLoaded) {
          emit(currentState.copyWith(
            guilds: [...currentState.guilds, guild],
            selectedGuild: guild,
          ));
          add(ChannelsLoadRequested(guild.id));
        }
      },
    );
  }

  Future<void> _onChannelsLoadRequested(
    ChannelsLoadRequested event,
    Emitter<GuildsState> emit,
  ) async {
    final currentState = state;
    if (currentState is! GuildsLoaded) return;

    final result = await _guildsRepository.getChannels(event.guildId);
    result.fold(
      (failure) => emit(GuildsError(failure.message)),
      (channels) {
        emit(currentState.copyWith(channels: channels));
        // Auto-select first text channel
        final textChannel = channels.where((c) => c.type == ChannelType.text).firstOrNull;
        if (textChannel != null) {
          add(ChannelSelected(textChannel.id));
        }
      },
    );
  }

  Future<void> _onChannelSelected(
    ChannelSelected event,
    Emitter<GuildsState> emit,
  ) async {
    final currentState = state;
    if (currentState is! GuildsLoaded) return;

    final channel = currentState.channels.firstWhere(
      (c) => c.id == event.channelId,
      orElse: () => currentState.channels.first,
    );

    emit(currentState.copyWith(selectedChannel: channel));
  }

  Future<void> _onChannelCreateRequested(
    ChannelCreateRequested event,
    Emitter<GuildsState> emit,
  ) async {
    final result = await _guildsRepository.createChannel(
      event.guildId,
      event.name,
      event.type,
    );
    result.fold(
      (failure) => emit(GuildsError(failure.message)),
      (channel) {
        final currentState = state;
        if (currentState is GuildsLoaded) {
          emit(currentState.copyWith(
            channels: [...currentState.channels, channel],
            selectedChannel: channel,
          ));
        }
      },
    );
  }
}
