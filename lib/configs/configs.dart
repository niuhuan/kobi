import 'package:kobi/configs/login.dart';
import 'package:kobi/configs/proxy.dart';
import 'package:kobi/configs/reader_controller_type.dart';
import 'package:kobi/configs/reader_direction.dart';
import 'package:kobi/configs/reader_slider_position.dart';
import 'package:kobi/configs/reader_type.dart';
import 'package:kobi/configs/versions.dart';

import 'cache_time.dart';
import 'collect_ordering.dart';

Future initConfigs() async {
  await initProxy();
  await initCacheTime();
  await initReaderControllerType();
  await initReaderDirection();
  await initReaderSliderPosition();
  await initReaderType();
  await initLogin();
  await initVersion();
  await collectOrderingSetting.initConfig();
  autoCheckNewVersion();
}
