import 'package:flutter/material.dart';
import 'package:kobi/configs/collect_ordering.dart';

import '../configs/cache_time.dart';
import '../configs/proxy.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: ListView(
        children: [
          proxySetting(),
          cacheTimeNameSetting(),
          collectOrderingSetting.configWidget(context),
        ],
      ),
    );
  }
}
