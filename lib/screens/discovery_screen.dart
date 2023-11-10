import 'package:flutter/material.dart';
import 'package:kobi/bridge_generated.dart';
import 'package:kobi/screens/components/content_error.dart';
import 'package:flutter_search_bar/flutter_search_bar.dart' as sb;
import '../ffi.io.dart';

class DiscoveryScreen extends StatefulWidget {
  const DiscoveryScreen({Key? key}) : super(key: key);

  @override
  _DiscoveryScreenState createState() => _DiscoveryScreenState();
}

class _DiscoveryScreenState extends State<DiscoveryScreen> {
  String _keyTop = "";
  String _keyTheme = "";
  String _keyOrdering = "";

  int _tagsLoadStatus = 0; // 0 : 加载中 , 1 : 加载成功 , 2 : 加载失败
  late UITags uiTags;

  _loadTags() async {
    try {
      setState(() {
        _tagsLoadStatus = 0;
      });
      uiTags = await api.tags();
      setState(() {
        _tagsLoadStatus = 1;
      });
    } catch (e, s) {
      setState(() {
        _tagsLoadStatus = 2;
      });
    }
  }

  late final _searchBar = sb.SearchBar(
    hintText: '搜索',
    inBar: false,
    setState: setState,
    onSubmitted: (value) {
      if (value.isNotEmpty) {
        // Navigator.push(
        //   context,
        //   mixRoute(
        //     builder: (context) => SearchScreen(keyword: value),
        //   ),
        // );
      }
    },
    buildDefaultAppBar: _buildNormalAppBar,
  );

  Widget _buildNormalAppBar(BuildContext context) {
    if (_tagsLoadStatus == 0) {
      return AppBar(
        title: const Text('加载中'),
        actions: [
          _searchBar.getSearchAction(context),
        ],
      );
    } else if (_tagsLoadStatus == 2) {
      return AppBar(
        title: MaterialButton(
          onPressed: _loadTags,
          child: const Text("分类加载失败, 点击重试"),
        ),
        actions: [
          _searchBar.getSearchAction(context),
        ],
      );
    } else if (_tagsLoadStatus == 1) {
      // ordering
      String orderingTitle = "默认";
      if (_keyOrdering != "") {
        for (var ordering in uiTags.ordering) {
          if (ordering.pathWord == _keyOrdering) {
            orderingTitle = ordering.name;
            break;
          }
        }
      }
      final orderingButton = PopupMenuButton<String>(
        child: Text(orderingTitle),
        itemBuilder: (BuildContext context) {
          List<PopupMenuItem<String>> orderingItems = [];
          orderingItems.add(const PopupMenuItem<String>(
            value: "",
            child: ListTile(
              title: Text("默认"),
            ),
          ));
          for (var ordering in uiTags.ordering) {
            orderingItems.add(PopupMenuItem<String>(
              value: ordering.pathWord,
              child: ListTile(
                title: Text(ordering.name),
              ),
            ));
          }
          return orderingItems;
        },
        onSelected: (String value) {
          setState(() {
            _keyOrdering = value;
          });
        },
      );
      // top
      String topTitle = "全部";
      if (_keyTop != "") {
        for (var top in uiTags.top) {
          if (top.pathWord == _keyTop) {
            topTitle = top.name;
            break;
          }
        }
      }
      final topButton = PopupMenuButton<String>(
        child: Text(topTitle),
        itemBuilder: (BuildContext context) {
          List<PopupMenuItem<String>> topItems = [];
          topItems.add(const PopupMenuItem<String>(
            value: "",
            child: ListTile(
              title: Text("全部"),
            ),
          ));
          for (var top in uiTags.top) {
            topItems.add(PopupMenuItem<String>(
              value: top.pathWord,
              child: ListTile(
                title: Text(top.name),
              ),
            ));
          }
          return topItems;
        },
        onSelected: (String value) {
          setState(() {
            _keyTop = value;
          });
        },
      );
      // theme
      String themeTitle = "全部";
      if (_keyTheme != "") {
        for (var theme in uiTags.theme) {
          if (theme.pathWord == _keyTheme) {
            themeTitle = theme.name;
            break;
          }
        }
      }
      final themeButton = PopupMenuButton<String>(
        child: Text(themeTitle),
        itemBuilder: (BuildContext context) {
          List<PopupMenuItem<String>> themeItems = [];
          themeItems.add(const PopupMenuItem<String>(
            value: "",
            child: ListTile(
              title: Text("全部"),
            ),
          ));
          for (var theme in uiTags.theme) {
            themeItems.add(PopupMenuItem<String>(
              value: theme.pathWord,
              child: ListTile(
                title: Text(theme.name),
              ),
            ));
          }
          return themeItems;
        },
        onSelected: (String value) {
          setState(() {
            _keyTheme = value;
          });
        },
      );
      // return
      return AppBar(
        title: Row(children: [
          Container(
            padding: const EdgeInsets.only(left: 5, right: 5),
            child: orderingButton,
          ),
          Container(
            padding: const EdgeInsets.only(left: 5, right: 5),
            child: topButton,
          ),
          Container(
            padding: const EdgeInsets.only(left: 5, right: 5),
            child: themeButton,
          ),
        ]),
        actions: [
          _searchBar.getSearchAction(context),
        ],
      );
    }
    return AppBar();
  }

  @override
  void initState() {
    super.initState();
    _loadTags();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _searchBar.build(context),
    );
  }
}
