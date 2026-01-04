/// Application configuration
class AppConfig {
  static const String apiBaseUrl = String.fromEnvironment(
    'API_BASE_URL',
    defaultValue: 'http://localhost:8080/api/v1',
  );

  static const String wsBaseUrl = String.fromEnvironment(
    'WS_BASE_URL',
    defaultValue: 'ws://localhost:8080/ws',
  );

  static const String hCaptchaSiteKey = String.fromEnvironment(
    'HCAPTCHA_SITE_KEY',
    defaultValue: '10000000-ffff-ffff-ffff-000000000001', // Test key
  );
}
