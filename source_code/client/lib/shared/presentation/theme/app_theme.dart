import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

class AppTheme {
  static ThemeData get darkTheme => ThemeData(
        useMaterial3: true,
        brightness: Brightness.dark,
        colorScheme: const ColorScheme.dark(
          primary: Color(0xFF5865F2), // Discord blurple
          secondary: Color(0xFF57F287), // Green
          surface: Color(0xFF313338), // Channel background
          error: Color(0xFFED4245), // Red
        ),
        scaffoldBackgroundColor: const Color(0xFF313338),
        cardTheme: const CardTheme(
          color: Color(0xFF2B2D31),
          elevation: 0,
        ),
        appBarTheme: const AppBarTheme(
          backgroundColor: Color(0xFF2B2D31),
          elevation: 0,
        ),
        inputDecorationTheme: InputDecorationTheme(
          filled: true,
          fillColor: const Color(0xFF383A40),
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(8),
            borderSide: BorderSide.none,
          ),
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 16,
            vertical: 12,
          ),
        ),
        elevatedButtonTheme: ElevatedButtonThemeData(
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF5865F2),
            foregroundColor: Colors.white,
            padding: const EdgeInsets.symmetric(
              horizontal: 24,
              vertical: 12,
            ),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
          ),
        ),
        textTheme: GoogleFonts.interTextTheme(
          ThemeData.dark().textTheme,
        ),
      );

  static const Color serverSidebarBackground = Color(0xFF1E1F22);
  static const Color channelSidebarBackground = Color(0xFF2B2D31);
  static const Color chatBackground = Color(0xFF313338);
  static const Color memberSidebarBackground = Color(0xFF2B2D31);
  static const Color inputBackground = Color(0xFF383A40);
  static const Color blurple = Color(0xFF5865F2);
  static const Color green = Color(0xFF57F287);
  static const Color yellow = Color(0xFFFEE75C);
  static const Color red = Color(0xFFED4245);
}
