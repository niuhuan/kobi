import 'package:kobi/configs/app_orientation.dart';
import 'package:kobi/configs/app_theme.dart';
import 'package:kobi/configs/comic_grid_columns.dart';
import 'package:kobi/configs/comic_pager_type.dart';
import 'package:kobi/configs/login.dart';
import 'package:kobi/configs/no_pager_animation.dart';
import 'package:kobi/configs/proxy.dart';
import 'package:kobi/configs/reader_controller_type.dart';
import 'package:kobi/configs/reader_direction.dart';
import 'package:kobi/configs/reader_slider_position.dart';
import 'package:kobi/configs/reader_type.dart';
import 'package:kobi/configs/versions.dart';

import 'api_host.dart';
import 'cache_time.dart';
import 'collect_ordering.dart';

Future initConfigs() async {
  await initAppOrientation();
  await initAppTheme();
  await initApiHost();
  await initProxy();
  await initCacheTime();
  await initReaderControllerType();
  await initReaderDirection();
  await initReaderSliderPosition();
  await initReaderType();
  await initLogin();
  await initVersion();
  await initNoPagerAnimation();
  await initComicPagerType();
  await initComicGridColumns();
  autoCheckNewVersion();
}
