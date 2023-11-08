
import 'package:kobi/configs/proxy.dart';

import 'cache_time.dart';

Future initConfigs() async {
  await initProxy();
  await initCacheTime();
}