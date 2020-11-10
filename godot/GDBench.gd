extends CanvasItem

export var cnt = 4000
export var start = Vector2(250,250)
export var rad = 200

func _draw():
	var startTime = OS.get_ticks_usec()

	var cntf = float(cnt)
	for n in range(cnt):
		var x = sin(n/cntf * 360.0)*rad
		var y = cos(n/cntf * 360.0)*rad
		draw_line(
			start, 
			start+Vector2(x, y), 
			Color(255, 0, 0), 
			1,
			false)

	print("bench: " + String(OS.get_ticks_usec() - startTime))

func _process(_delta):
	update()
