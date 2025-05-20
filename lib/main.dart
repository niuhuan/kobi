import 'package:flutter/material.dart' hide Size;
import 'package:event/event.dart';
import 'package:kobi/configs/app_theme.dart';
import 'package:kobi/src/rust/frb_generated.dart';
import 'package:kobi/screens/components/router.dart';
import 'screens/init_screen.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  void initState() {
    super.initState();
    appThemeEvent.subscribe(_onThemeChange);
  }

  @override
  void dispose() {
    appThemeEvent.unsubscribe(_onThemeChange);
    super.dispose();
  }

  void _onThemeChange(AppThemeEventArgs? args) {
    if (args != null) {
      setState(() {});
    }
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorObservers: [
        routeObserver,
      ],
      theme: _lightTheme,
      darkTheme: _darkTheme,
      themeMode: _getThemeMode(),
      home: const InitScreen(),
    );
  }

  ThemeMode _getThemeMode() {
    switch (currentAppTheme) {
      case AppTheme.system:
        return ThemeMode.system;
      case AppTheme.light:
        return ThemeMode.light;
      case AppTheme.dark:
        return ThemeMode.dark;
    }
  }
}

final _defaultLight = ThemeData.light();
final _defaultDark = ThemeData.dark();

final _lightTheme = _defaultLight.copyWith(
  appBarTheme: _defaultLight.appBarTheme.copyWith(
    backgroundColor: Color.alphaBlend(
      Colors.redAccent.shade700.withOpacity(.05),
      _defaultLight.scaffoldBackgroundColor,
    ),
    foregroundColor: _defaultLight.textTheme.bodyMedium?.color,
    elevation: 0.5,
  ),
  tabBarTheme: _defaultLight.tabBarTheme.copyWith(
    labelColor: Colors.red.shade600,
    unselectedLabelColor: Colors.grey.shade600,
    labelStyle: TextStyle(
      fontSize: 12,
      fontWeight: FontWeight.w900,
      color: Colors.red.shade600,
    ),
    unselectedLabelStyle: const TextStyle(
      fontSize: 12,
      fontWeight: FontWeight.w500,
    ),
    indicator: const UnderlineTabIndicator(
      borderSide: BorderSide(
        width: 2,
        color: Colors.red,
      ),
    ),
  ),
  navigationBarTheme: _defaultLight.navigationBarTheme.copyWith(
    indicatorColor: Colors.transparent,
    backgroundColor: Colors.white,
    labelTextStyle: MaterialStateProperty.resolveWith((state) {
      if (state.contains(MaterialState.selected)) {
        return TextStyle(
          fontSize: 12,
          color: Colors.red.shade600,
          fontWeight: FontWeight.w500,
        );
      } else {
        return TextStyle(
          fontSize: 12,
          color: Colors.grey.shade600,
          fontWeight: FontWeight.w500,
        );
      }
    }),
    iconTheme: MaterialStateProperty.resolveWith((state) {
      if (state.contains(MaterialState.selected)) {
        return IconThemeData(
          size: 24,
          color: Colors.red.shade600,
        );
      } else {
        return IconThemeData(
          size: 24,
          color: Colors.grey.shade600,
        );
      }
    }),
  ),
);

final _darkTheme = _defaultDark.copyWith(
  appBarTheme: _defaultDark.appBarTheme.copyWith(
    backgroundColor: Color.alphaBlend(
      Colors.redAccent.shade700.withOpacity(.05),
      _defaultDark.scaffoldBackgroundColor,
    ),
    foregroundColor: _defaultDark.textTheme.bodyMedium?.color,
    elevation: 0.5,
  ),
  tabBarTheme: _defaultDark.tabBarTheme.copyWith(
    labelColor: Colors.red.shade400,
    unselectedLabelColor: Colors.grey.shade400,
    labelStyle: TextStyle(
      fontSize: 12,
      fontWeight: FontWeight.w900,
      color: Colors.red.shade400,
    ),
    unselectedLabelStyle: const TextStyle(
      fontSize: 12,
      fontWeight: FontWeight.w500,
    ),
    indicator: const UnderlineTabIndicator(
      borderSide: BorderSide(
        width: 2,
        color: Colors.red,
      ),
    ),
  ),
  navigationBarTheme: _defaultDark.navigationBarTheme.copyWith(
    indicatorColor: Colors.transparent,
    backgroundColor: Colors.grey[900],
    labelTextStyle: MaterialStateProperty.resolveWith((state) {
      if (state.contains(MaterialState.selected)) {
        return TextStyle(
          fontSize: 12,
          color: Colors.red.shade400,
          fontWeight: FontWeight.w500,
        );
      } else {
        return TextStyle(
          fontSize: 12,
          color: Colors.grey.shade400,
          fontWeight: FontWeight.w500,
        );
      }
    }),
    iconTheme: MaterialStateProperty.resolveWith((state) {
      if (state.contains(MaterialState.selected)) {
        return IconThemeData(
          size: 24,
          color: Colors.red.shade400,
        );
      } else {
        return IconThemeData(
          size: 24,
          color: Colors.grey.shade400,
        );
      }
    }),
  ),
);
