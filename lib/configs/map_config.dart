import 'package:event/event.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:kobi/src/rust/api/api.dart';

import '../screens/components/commons.dart';

class MapConfig {
  late final Map<String, String> valueName;
  late final Map<String, String> nameValue;
  final String propertyName;
  final String propertyKey;
  late String value;
  late final Event changeEvent;
  final String defaultValue;

  MapConfig({
    required Map<String, String> valueName,
    required this.defaultValue,
    required this.propertyKey,
    required this.propertyName,
  }) {
    this.valueName = valueName;
    this.nameValue = {};
    valueName.forEach((key, value) {
      nameValue[value] = key;
    });
    this.changeEvent = Event();
    if (!valueName.containsKey(defaultValue)) {
      throw ArgumentError("defaultValue not in valueName");
    }
  }

  Future initConfig() async {
    value = await loadProperty(k: propertyKey);
    if (!valueName.containsKey(value)) {
      value = defaultValue;
      await saveProperty(k: propertyKey, v: value);
    }
  }

  Widget configWidget(BuildContext context) {
    return StatefulBuilder(
      builder: (BuildContext context, void Function(void Function()) setState) {
        return ListTile(
          title: Text(propertyName),
          subtitle: Text(valueName[value] ?? ""),
          onTap: () async {
            String? result = await chooseMapDialog<String>(
              context,
              title: propertyName,
              values: nameValue,
            );
            if (result != null) {
              await saveProperty(k: propertyKey, v: result);
              value = result;
              setState(() {});
              changeEvent.broadcast();
            }
          },
        );
      },
    );
  }
}
