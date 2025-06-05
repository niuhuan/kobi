import 'package:flutter/material.dart';
import 'package:event/event.dart';
import '../src/rust/api/api.dart' as api;

enum AppTheme {
  system, // 跟随系统
  light, // 浅色主题
  dark, // 深色主题
}

const _propertyName = "app_theme";

late AppTheme _appTheme = AppTheme.system;

AppTheme get currentAppTheme => _appTheme;

class AppThemeEventArgs extends EventArgs {
  final AppTheme theme;

  AppThemeEventArgs(this.theme);
}

final Event<AppThemeEventArgs> appThemeEvent = Event<AppThemeEventArgs>();

Future initAppTheme() async {
  var value = await api.loadProperty(k: _propertyName);
  if (value == null) {
    await api.saveProperty(k: _propertyName, v: AppTheme.system.name);
    _appTheme = AppTheme.system;
  } else {
    _appTheme = AppTheme.values.firstWhere(
      (e) => e.name == value,
      orElse: () => AppTheme.system,
    );
  }
  appThemeEvent.broadcast(AppThemeEventArgs(_appTheme));
}

Future chooseAppTheme(BuildContext context) async {
  var result = await showDialog<AppTheme>(
    context: context,
    builder: (BuildContext context) {
      return AlertDialog(
        backgroundColor: const Color(0xAA000000),
        title: const Text(
          "选择应用主题",
          style: TextStyle(color: Colors.white),
        ),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              title: const Text(
                "跟随系统",
                style: TextStyle(color: Colors.white),
              ),
              onTap: () {
                Navigator.of(context).pop(AppTheme.system);
              },
            ),
            ListTile(
              title: const Text(
                "浅色主题",
                style: TextStyle(color: Colors.white),
              ),
              onTap: () {
                Navigator.of(context).pop(AppTheme.light);
              },
            ),
            ListTile(
              title: const Text(
                "深色主题",
                style: TextStyle(color: Colors.white),
              ),
              onTap: () {
                Navigator.of(context).pop(AppTheme.dark);
              },
            ),
          ],
        ),
      );
    },
  );
  if (result != null) {
    await api.saveProperty(k: _propertyName, v: result.name);
    _appTheme = result;
    appThemeEvent.broadcast(AppThemeEventArgs(_appTheme));
  }
}

String appThemeName(AppTheme theme, BuildContext context) {
  switch (theme) {
    case AppTheme.system:
      return "跟随系统";
    case AppTheme.light:
      return "浅色主题";
    case AppTheme.dark:
      return "深色主题";
  }
}

Widget appThemeSetting(BuildContext context) {
  return StatefulBuilder(
    builder: (BuildContext context, void Function(void Function()) setState) {
      return ListTile(
        title: const Text(
          "应用主题",
        ),
        subtitle: Text(
          appThemeName(currentAppTheme, context),
        ),
        onTap: () async {
          await chooseAppTheme(context);
          setState(() {}); // 更新状态以反映新的主题
        },
      );
    },
  );
}
