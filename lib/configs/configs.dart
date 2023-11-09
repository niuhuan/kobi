
import 'package:kobi/configs/proxy.dart';
import 'package:kobi/configs/reader_controller_type.dart';
import 'package:kobi/configs/reader_direction.dart';
import 'package:kobi/configs/reader_slider_position.dart';
import 'package:kobi/configs/reader_type.dart';

import 'cache_time.dart';

Future initConfigs() async {
  await initProxy();
  await initCacheTime();
  await initReaderControllerType();
  await initReaderDirection();
  await initReaderSliderPosition();
  await initReaderType();
}