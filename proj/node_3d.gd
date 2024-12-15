extends Node3D

func _ready() -> void:
	$Node3D.get_script().reload()
	print($Node3D.funny)
