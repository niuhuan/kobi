import 'map_config.dart';

final collectOrderingSetting = MapConfig(
  valueName: {
    "-datetime_modifier": "收藏时间倒序",
    "datetime_modifier": "收藏时间正序",
    "-datetime_updated": "更新时间倒序",
    "datetime_updated": "更新时间正序",
  },
  defaultValue: "-datetime_modifier",
  propertyKey: "collect_ordering",
  propertyName: "收藏排序",
);
