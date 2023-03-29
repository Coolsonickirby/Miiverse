import bpy, mathutils

C = bpy.context
src_obj = bpy.context.active_object
last_obj_z = src_obj.location.z
last_obj_x = src_obj.location.x
x_step = 100
z_step = -50
x_uv_step = 0.2
y_uv_step = -0.2
last_x_uv = 0
last_y_uv = 0

amount_x = 8
amount_y = 8

for i in range (0,amount_y):
    for y in range(0, amount_x):
        new_obj = src_obj.copy()
        new_obj.data = src_obj.data.copy()
        new_obj.animation_data_clear()
        last_obj_x += x_step
        new_obj.location.x += last_obj_x
        new_obj.location.z = last_obj_z
        new_obj.name = f"{i} - {y}"
        # UV Loops
        for loop in new_obj.data.loops :
            print(loop.index)
            uv_coords = new_obj.data.uv_layers.active.data[loop.index].uv
            res = uv_coords + mathutils.Vector((last_x_uv, last_y_uv))
            print(res)
            new_obj.data.uv_layers.active.data[loop.index].uv = res
        last_x_uv += x_uv_step
        C.collection.objects.link(new_obj)
    last_obj_z += z_step
    last_obj_x = src_obj.location.x
    last_y_uv += y_uv_step
    last_x_uv = 0