import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';

import '../../../../shared/presentation/theme/app_theme.dart';
import '../../domain/entities/guild.dart';
import '../bloc/guilds_bloc.dart';

class GuildsPage extends StatefulWidget {
  final Widget child;

  const GuildsPage({super.key, required this.child});

  @override
  State<GuildsPage> createState() => _GuildsPageState();
}

class _GuildsPageState extends State<GuildsPage> {
  @override
  void initState() {
    super.initState();
    context.read<GuildsBloc>().add(GuildsLoadRequested());
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<GuildsBloc, GuildsState>(
      builder: (context, state) {
        return Scaffold(
          body: Row(
            children: [
              // Guild sidebar
              _GuildsSidebar(
                guilds: state is GuildsLoaded ? state.guilds : [],
                selectedGuildId: state is GuildsLoaded ? state.selectedGuild?.id : null,
                onGuildSelected: (guildId) {
                  context.read<GuildsBloc>().add(GuildSelected(guildId));
                  context.go('/guilds/$guildId');
                },
                onCreateGuild: () => _showCreateGuildDialog(context),
              ),

              // Channel sidebar
              if (state is GuildsLoaded && state.selectedGuild != null)
                _ChannelsSidebar(
                  guild: state.selectedGuild!,
                  channels: state.channels,
                  selectedChannelId: state.selectedChannel?.id,
                  onChannelSelected: (channelId) {
                    context.read<GuildsBloc>().add(ChannelSelected(channelId));
                    context.go('/guilds/${state.selectedGuild!.id}/channels/$channelId');
                  },
                  onCreateChannel: () => _showCreateChannelDialog(context, state.selectedGuild!.id),
                ),

              // Main content
              Expanded(child: widget.child),
            ],
          ),
        );
      },
    );
  }

  void _showCreateGuildDialog(BuildContext context) {
    final nameController = TextEditingController();
    final descController = TextEditingController();

    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: const Text('Create Server'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: nameController,
              decoration: const InputDecoration(
                labelText: 'Server Name',
                hintText: 'Enter server name',
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: descController,
              decoration: const InputDecoration(
                labelText: 'Description (optional)',
                hintText: 'Enter description',
              ),
              maxLines: 2,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () {
              if (nameController.text.isNotEmpty) {
                context.read<GuildsBloc>().add(GuildCreateRequested(
                      nameController.text,
                      descController.text.isEmpty ? null : descController.text,
                    ));
                Navigator.pop(ctx);
              }
            },
            child: const Text('Create'),
          ),
        ],
      ),
    );
  }

  void _showCreateChannelDialog(BuildContext context, String guildId) {
    final nameController = TextEditingController();
    var selectedType = ChannelType.text;

    showDialog(
      context: context,
      builder: (ctx) => StatefulBuilder(
        builder: (ctx, setDialogState) => AlertDialog(
          title: const Text('Create Channel'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              TextField(
                controller: nameController,
                decoration: const InputDecoration(
                  labelText: 'Channel Name',
                  hintText: 'Enter channel name',
                ),
              ),
              const SizedBox(height: 16),
              SegmentedButton<ChannelType>(
                segments: const [
                  ButtonSegment(
                    value: ChannelType.text,
                    icon: Icon(Icons.tag),
                    label: Text('Text'),
                  ),
                  ButtonSegment(
                    value: ChannelType.voice,
                    icon: Icon(Icons.volume_up),
                    label: Text('Voice'),
                  ),
                ],
                selected: {selectedType},
                onSelectionChanged: (types) {
                  setDialogState(() {
                    selectedType = types.first;
                  });
                },
              ),
            ],
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(ctx),
              child: const Text('Cancel'),
            ),
            ElevatedButton(
              onPressed: () {
                if (nameController.text.isNotEmpty) {
                  context.read<GuildsBloc>().add(ChannelCreateRequested(
                        guildId,
                        nameController.text,
                        selectedType,
                      ));
                  Navigator.pop(ctx);
                }
              },
              child: const Text('Create'),
            ),
          ],
        ),
      ),
    );
  }
}

class _GuildsSidebar extends StatelessWidget {
  final List<Guild> guilds;
  final String? selectedGuildId;
  final Function(String) onGuildSelected;
  final VoidCallback onCreateGuild;

  const _GuildsSidebar({
    required this.guilds,
    this.selectedGuildId,
    required this.onGuildSelected,
    required this.onCreateGuild,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 72,
      color: AppTheme.backgroundTertiary,
      child: Column(
        children: [
          const SizedBox(height: 12),
          // Home button
          _GuildIcon(
            icon: Icons.home,
            isSelected: selectedGuildId == null,
            onTap: () {},
          ),
          const Padding(
            padding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Divider(height: 2),
          ),
          // Guild list
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.symmetric(vertical: 4),
              itemCount: guilds.length,
              itemBuilder: (context, index) {
                final guild = guilds[index];
                return Padding(
                  padding: const EdgeInsets.symmetric(vertical: 4),
                  child: _GuildIcon(
                    label: guild.name.substring(0, 1).toUpperCase(),
                    isSelected: guild.id == selectedGuildId,
                    onTap: () => onGuildSelected(guild.id),
                    tooltip: guild.name,
                  ),
                );
              },
            ),
          ),
          const Padding(
            padding: EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Divider(height: 2),
          ),
          // Add guild button
          Padding(
            padding: const EdgeInsets.only(bottom: 12),
            child: _GuildIcon(
              icon: Icons.add,
              onTap: onCreateGuild,
              color: AppTheme.green,
            ),
          ),
        ],
      ),
    );
  }
}

class _GuildIcon extends StatelessWidget {
  final IconData? icon;
  final String? label;
  final bool isSelected;
  final VoidCallback onTap;
  final Color? color;
  final String? tooltip;

  const _GuildIcon({
    this.icon,
    this.label,
    this.isSelected = false,
    required this.onTap,
    this.color,
    this.tooltip,
  });

  @override
  Widget build(BuildContext context) {
    final widget = GestureDetector(
      onTap: onTap,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        width: 48,
        height: 48,
        decoration: BoxDecoration(
          color: isSelected ? AppTheme.blurple : AppTheme.backgroundSecondary,
          borderRadius: BorderRadius.circular(isSelected ? 16 : 24),
        ),
        child: Center(
          child: icon != null
              ? Icon(icon, color: color ?? Colors.white)
              : Text(
                  label ?? '',
                  style: TextStyle(
                    color: color ?? Colors.white,
                    fontWeight: FontWeight.bold,
                    fontSize: 18,
                  ),
                ),
        ),
      ),
    );

    if (tooltip != null) {
      return Tooltip(message: tooltip!, child: widget);
    }
    return widget;
  }
}

class _ChannelsSidebar extends StatelessWidget {
  final Guild guild;
  final List<Channel> channels;
  final String? selectedChannelId;
  final Function(String) onChannelSelected;
  final VoidCallback onCreateChannel;

  const _ChannelsSidebar({
    required this.guild,
    required this.channels,
    this.selectedChannelId,
    required this.onChannelSelected,
    required this.onCreateChannel,
  });

  @override
  Widget build(BuildContext context) {
    final textChannels = channels.where((c) => c.type == ChannelType.text).toList();
    final voiceChannels = channels.where((c) => c.type == ChannelType.voice).toList();

    return Container(
      width: 240,
      color: AppTheme.backgroundSecondary,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Guild header
          Container(
            height: 48,
            padding: const EdgeInsets.symmetric(horizontal: 16),
            decoration: BoxDecoration(
              border: Border(
                bottom: BorderSide(color: Colors.black.withValues(alpha: 0.2)),
              ),
            ),
            child: Row(
              children: [
                Expanded(
                  child: Text(
                    guild.name,
                    style: const TextStyle(
                      fontWeight: FontWeight.bold,
                      fontSize: 16,
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.expand_more, size: 20),
                  onPressed: () {},
                  padding: EdgeInsets.zero,
                  constraints: const BoxConstraints(),
                ),
              ],
            ),
          ),

          // Channels list
          Expanded(
            child: ListView(
              padding: const EdgeInsets.symmetric(vertical: 8),
              children: [
                // Text channels
                _ChannelCategory(
                  title: 'TEXT CHANNELS',
                  onAdd: onCreateChannel,
                ),
                ...textChannels.map((channel) => _ChannelItem(
                      channel: channel,
                      isSelected: channel.id == selectedChannelId,
                      onTap: () => onChannelSelected(channel.id),
                    )),

                const SizedBox(height: 16),

                // Voice channels
                _ChannelCategory(
                  title: 'VOICE CHANNELS',
                  onAdd: onCreateChannel,
                ),
                ...voiceChannels.map((channel) => _ChannelItem(
                      channel: channel,
                      isSelected: channel.id == selectedChannelId,
                      onTap: () => onChannelSelected(channel.id),
                    )),
              ],
            ),
          ),

          // User area
          Container(
            height: 52,
            padding: const EdgeInsets.symmetric(horizontal: 8),
            decoration: BoxDecoration(
              color: AppTheme.backgroundTertiary,
            ),
            child: Row(
              children: [
                const CircleAvatar(
                  radius: 16,
                  backgroundColor: AppTheme.blurple,
                  child: Icon(Icons.person, size: 20, color: Colors.white),
                ),
                const SizedBox(width: 8),
                const Expanded(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Username',
                        style: TextStyle(fontSize: 14, fontWeight: FontWeight.w500),
                      ),
                      Text(
                        'Online',
                        style: TextStyle(fontSize: 12, color: Colors.grey),
                      ),
                    ],
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.settings, size: 20),
                  onPressed: () {},
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _ChannelCategory extends StatelessWidget {
  final String title;
  final VoidCallback onAdd;

  const _ChannelCategory({
    required this.title,
    required this.onAdd,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      child: Row(
        children: [
          Expanded(
            child: Text(
              title,
              style: TextStyle(
                fontSize: 12,
                fontWeight: FontWeight.bold,
                color: Colors.grey.shade400,
                letterSpacing: 0.5,
              ),
            ),
          ),
          IconButton(
            icon: Icon(Icons.add, size: 16, color: Colors.grey.shade400),
            onPressed: onAdd,
            padding: EdgeInsets.zero,
            constraints: const BoxConstraints(),
          ),
        ],
      ),
    );
  }
}

class _ChannelItem extends StatelessWidget {
  final Channel channel;
  final bool isSelected;
  final VoidCallback onTap;

  const _ChannelItem({
    required this.channel,
    required this.isSelected,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: isSelected ? AppTheme.backgroundModifierSelected : Colors.transparent,
      borderRadius: BorderRadius.circular(4),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(4),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
          child: Row(
            children: [
              Icon(
                channel.type == ChannelType.text ? Icons.tag : Icons.volume_up,
                size: 20,
                color: Colors.grey.shade400,
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Text(
                  channel.name,
                  style: TextStyle(
                    color: isSelected ? Colors.white : Colors.grey.shade300,
                    fontWeight: isSelected ? FontWeight.w500 : FontWeight.normal,
                  ),
                  overflow: TextOverflow.ellipsis,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
