import 'package:flutter/material.dart';
import 'package:kobi/screens/components/commnet_card.dart';
import 'package:kobi/screens/components/content_loading.dart';
import 'package:kobi/src/rust/api/api.dart' as api;
import 'package:kobi/src/rust/udto.dart';

import 'content_error.dart';

class CommnetList extends StatefulWidget {
  final String comicId;
  final int? parentId;

  const CommnetList(this.comicId, {Key? key, this.parentId}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _CommnetListState();
}

class _CommnetListState extends State<CommnetList> {
  late int _commentOffset;
  late Future<UIPageComment> _commentFuture;

  @override
  void initState() {
    _load(0);
    super.initState();
  }

  _load(int offser) {
    setState(() {
      _commentOffset = offser;
      _commentFuture = api.comments(
        comicId: widget.comicId,
        offset: BigInt.from(_commentOffset),
        limit: BigInt.from(10),
        replyId: widget.parentId?.toString() ?? "",
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _commentFuture,
      builder: (BuildContext context, AsyncSnapshot<UIPageComment> snapshot) {
        if (snapshot.hasError) {
          return ContentError(
            onRefresh: () async {
              setState(() {
                _commentFuture = api.comments(
                    comicId: widget.comicId,
                    offset: BigInt.from(_commentOffset),
                    limit: BigInt.from(10));
              });
            },
            error: snapshot.error,
            stackTrace: snapshot.stackTrace,
            sq: true,
          );
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return ContentLoading(sq: true);
        }
        var data = snapshot.requireData;
        return Column(
          children: [
            if (data.offset > 0)
              TextButton(
                onPressed: () {
                  _load(data.offset - data.limit);
                },
                child: const Text('上一页'),
              ),
            ...data.list.map((e) {
              return GestureDetector(
                onTap: () {
                  if (widget.parentId != null) {
                    return;
                  }
                  Navigator.of(context)
                      .push(MaterialPageRoute(builder: (context) {
                    return Scaffold(
                      appBar: AppBar(
                        title: const Text('评论'),
                      ),
                      body: Column(
                        children: [
                          CommentCard(e),
                          Expanded(
                            child: ListView(
                              children: [
                                CommnetList(widget.comicId, parentId: e.id),
                              ],
                            ),
                          ),
                        ],
                      ),
                    );
                  }));
                },
                child: CommentCard(e),
              );
            }),
            if (data.offset + data.limit < data.total)
              TextButton(
                onPressed: () {
                  _load(data.offset + data.limit);
                },
                child: const Text('下一页'),
              ),
          ],
        );
      },
    );
  }
}
