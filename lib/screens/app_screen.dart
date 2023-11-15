import 'package:flutter/material.dart';
import 'rank_screen.dart';
import 'user_screen.dart';
import 'discovery_screen.dart';

class AppScreen extends StatefulWidget {
  const AppScreen({Key? key}) : super(key: key);

  @override
  State<AppScreen> createState() => _AppScreenState();
}

class _AppScreenState extends State<AppScreen> {
  var _pageIndex = 1;
  late final _pageController = PageController(initialPage: _pageIndex);

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    _pageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      body: PageView(
        physics: const NeverScrollableScrollPhysics(),
        allowImplicitScrolling: false,
        controller: _pageController,
        onPageChanged: (index) {
          /// 重新渲染导航
          setState(() {
            _pageIndex = index;
          });
        },
        children: _screens.map((e) => e.screen).toList(),
      ),
      bottomNavigationBar: BottomNavigationBar(
        items: _screens
            .map((e) => BottomNavigationBarItem(
                  label: e.title,
                  icon: Icon(e.icon),
                  tooltip: "",
                ))
            .toList(),
        currentIndex: _pageIndex,
        onTap: _onItemTapped,
        selectedLabelStyle: const TextStyle(
          fontSize: 12,
        ),
        unselectedLabelStyle: const TextStyle(
          fontSize: 12,
        ),
        selectedItemColor: theme.tabBarTheme.labelColor,
        unselectedItemColor: theme.tabBarTheme.unselectedLabelColor,
        showSelectedLabels: true,
        showUnselectedLabels: true,
        iconSize: 24,
        type: BottomNavigationBarType.fixed,
      ),
    );
  }

  /// 导航内容
  late final List<AppScreenData> _screens = const [
    AppScreenData(
      RankScreen(),
      '排行',
      Icons.local_fire_department_outlined,
    ),
    AppScreenData(
      DiscoveryScreen(),
      '发现',
      Icons.filter_list_sharp,
    ),
    AppScreenData(
      UserScreen(),
      '偏好',
      Icons.account_box_outlined,
    ),
  ];

  void _onItemTapped(int value) {
    setState(() {
      _pageIndex = value;
    });
    _pageController.jumpToPage(
      value,
    );
  }
}

class AppScreenData {
  final Widget screen;
  final String title;
  final IconData icon;

  const AppScreenData(this.screen, this.title, this.icon);
}
