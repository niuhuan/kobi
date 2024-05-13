
import 'dart:convert';

import 'package:kobi/src/rust/copy_client/dtos.dart';


List<Author> stringAuthors(String data) {
  return mapAuthor(List.of(jsonDecode(data)).cast());
}

List<Author> mapAuthor(List<Map> list) {
  List<Author> result = [];
  for (var value in list) {
    if (value['name'] != null && value['path_word'] != null) {
      result.add(Author(
        name: value['name'],
        pathWord: value['path_word'],
      ));
    }
  }
  return result;
}

ClassifyItem stringClassifyItem(String data) {
  return mapClassifyItem(jsonDecode(data));
}

ClassifyItem mapClassifyItem(Map map) {
  return ClassifyItem(
    display: map['display'],
    value: map['value'],
  );
}
