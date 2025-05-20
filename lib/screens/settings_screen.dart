import 'package:flutter/material.dart';
import 'package:kobi/configs/api_host.dart';
import 'package:kobi/configs/app_orientation.dart';
import 'package:kobi/configs/collect_ordering.dart';
import 'package:kobi/configs/comic_grid_columns.dart';
import 'package:kobi/configs/comic_pager_type.dart';
import 'package:kobi/configs/no_pager_animation.dart';

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
          apiHostSetting(),
          proxySetting(),
          cacheTimeNameSetting(),
          appOrientationWidget(),
          noPagerAnimationSwitch(),
          comicPagerTypeSetting(context),
          comicGridColumnsSetting(context),
        ],
      ),
    );
  }
}
