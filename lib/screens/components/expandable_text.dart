import 'package:flutter/material.dart';

class ExpandableText extends StatefulWidget {
  final String text;
  final int maxLines;
  final TextStyle style;
  final bool expand;

  const ExpandableText(this.text,
      {Key? key,
      required this.maxLines,
      required this.style,
      required this.expand})
      : super(key: key);

  @override
  State<StatefulWidget> createState() {
    return _ExpandableTextState();
  }
}

class _ExpandableTextState extends State<ExpandableText> {
  late bool expand = widget.expand;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, size) {
      final span = TextSpan(text: widget.text, style: widget.style);
      final tp = TextPainter(
        text: span,
        maxLines: widget.maxLines,
        textDirection: TextDirection.ltr,
      );
      tp.layout(maxWidth: size.maxWidth);

      final max = TextPainter(
        text: span,
        textDirection: TextDirection.ltr,
      );
      max.layout(maxWidth: size.maxWidth);

      if (tp.didExceedMaxLines) {
        return GestureDetector(
          behavior: HitTestBehavior.translucent,
          onTap: () {
            setState(() {
              expand = !expand;
            });
          },
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              expand
                  ? Stack(children: [
                      Text(widget.text, style: widget.style),
                      SizedBox(
                        height: max.height,
                        child: Align(
                          alignment: Alignment.bottomRight,
                          child: Transform.rotate(
                            angle: -90 * 3.1415926535 / 180,
                            child: Icon(
                              Icons.arrow_forward_ios,
                              size: widget.style.fontSize,
                              color: Colors.grey,
                            ),
                          ),
                        ),
                      ),
                    ])
                  : Stack(children: [
                      Text(
                        widget.text,
                        maxLines: widget.maxLines,
                        overflow: TextOverflow.ellipsis,
                        style: widget.style,
                      ),
                      SizedBox(
                        height: tp.height,
                        child: Align(
                          alignment: Alignment.bottomRight,
                          child: Container(
                            height: (widget.style.fontSize ?? 14) *
                                (widget.style.height ?? 1.3),
                            width: (widget.style.fontSize ?? 14) * 3,
                            decoration: const BoxDecoration(
                              gradient: LinearGradient(
                                end: Alignment.centerRight,
                                begin: Alignment.centerLeft,
                                colors: [
                                  Color(0x00ffffff),
                                  Colors.white,
                                ],
                              ),
                            ),
                            child: Align(
                              alignment: Alignment.centerRight,
                              child: Transform.rotate(
                                angle: 90 * 3.1415926535 / 180,
                                child: Icon(
                                  Icons.arrow_forward_ios,
                                  size: widget.style.fontSize,
                                  color: Colors.grey,
                                ),
                              ),
                            ),
                          ),
                        ),
                      ),
                    ]),
            ],
          ),
        );
      } else {
        return Text(widget.text, style: widget.style);
      }
    });
  }
}
