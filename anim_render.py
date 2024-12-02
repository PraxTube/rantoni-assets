# NOTE: You must set 'Make Markers Local' in blender in the animation marker section.
# Otherwise you will not be able to send events using markers.

from math import pi, sqrt

import bpy

# Camera settings
Z_HEIGHT = 9
X_DISTANCE = 9
X_ANGLE = 0.95
SQRT_2 = sqrt(2)
SUN_ANGLES = (-0.38, 0.19, 0)

# Names for objects in Blender.
# It is required to name them this way, they are also the default name.
CAMERA_STRING_NAME = "Camera"
SUN_LIGHT_STRING_NAME = "Sun"
CHARACTER_RIG_NAME = "rig"

RENDER_RESOLUTION_X = 125
RENDER_RESOLUTION_Y = 125
COLOR_PALETTE_FILE = "./palette.png"

# Output files
RENDER_OUTPUT_DIR = "render"
METADATA_OUTPUT_DIR = "metadata.csv"

bpy.context.scene.render.resolution_x = RENDER_RESOLUTION_X
bpy.context.scene.render.resolution_y = RENDER_RESOLUTION_Y
bpy.context.scene.render.fps = 18

# Make sure the output has no anit-aliasing (make it pixel perfect)
bpy.context.scene.render.film_transparent = True
bpy.context.scene.render.filter_size = 0.0001

print("\n\n---> Attempting to render animations...")

camera_positions_rotations = [
    # Top
    ((0, X_DISTANCE, Z_HEIGHT), (X_ANGLE, 0, pi)),
    # Top Right
    ((-X_DISTANCE / SQRT_2, X_DISTANCE / SQRT_2, Z_HEIGHT), (X_ANGLE, 0, 5 / 4 * pi)),
    # Right
    ((-X_DISTANCE, 0, Z_HEIGHT), (X_ANGLE, 0, - pi / 2)),
    # Bottom Right
    ((-X_DISTANCE / SQRT_2, -X_DISTANCE / SQRT_2, Z_HEIGHT), (X_ANGLE, 0, - 1 / 4 * pi)),
    # Bottom
    ((0, -X_DISTANCE, Z_HEIGHT), (X_ANGLE, 0, 0)),
    # Bottom Left
    ((X_DISTANCE / SQRT_2, -X_DISTANCE / SQRT_2, Z_HEIGHT), (X_ANGLE, 0, 1 / 4 * pi)),
    # Left
    ((X_DISTANCE, 0, Z_HEIGHT), (X_ANGLE, 0, pi / 2)),
    # Top Left
    ((X_DISTANCE / SQRT_2, X_DISTANCE / SQRT_2, Z_HEIGHT), (X_ANGLE, 0, -5 / 4 * pi)),
]

camera = bpy.data.objects[CAMERA_STRING_NAME]
if camera is None:
    raise ValueError(f"Expected to find camera with name '{CAMERA_STRING_NAME}' but none is present in scene.")

sun = bpy.data.objects[SUN_LIGHT_STRING_NAME]
if sun is None:
    raise ValueError(f"Expected to find light with name '{SUN_LIGHT_STRING_NAME}' but none is present in scene.")

rig = bpy.data.objects[CHARACTER_RIG_NAME]
if rig is None:
    raise ValueError(f"Expected to find rig with name '{CHARACTER_RIG_NAME}' but none is present in scene.")

metadata = []

for action in bpy.data.actions:
    print(f"\n\n---> Rendering {action.name} now...")
    rig.animation_data.action = bpy.data.actions.get(action.name)

    for marker in action.pose_markers:
        metadata.append(f"{action.name},{marker.name},{marker.frame}\n")

    bpy.context.scene.frame_start = int(action.frame_start)
    bpy.context.scene.frame_end = int(action.frame_end)

    if bpy.context.scene.frame_start == bpy.context.scene.frame_end:
        raise ValueError("Animation has only a single frame. This is most likely not intended. You most likely forgot to set the 'Manual Frame Range' in the dropsheet Action Editor menu.")

    for (orientation, (position, rotation)) in enumerate(camera_positions_rotations):
        camera.location = position
        camera.rotation_euler = rotation
        sun.rotation_euler = (SUN_ANGLES[0], SUN_ANGLES[1], orientation / 8 * 2 * pi)

        bpy.context.scene.render.filepath = f"{RENDER_OUTPUT_DIR}/{action.name}-o{orientation}-"
        bpy.ops.render.render(animation=True)

with open(METADATA_OUTPUT_DIR, "w") as f:
    f.writelines(metadata)
