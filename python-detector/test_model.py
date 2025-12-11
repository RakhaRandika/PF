from ultralytics import YOLO
import sys

# Load model
model = YOLO('../best (3).pt')

# Print model info
print("=" * 50)
print("MODEL INFORMATION")
print("=" * 50)
print(f"Model type: {type(model)}")
print(f"Model task: {model.task if hasattr(model, 'task') else 'Unknown'}")

# Get class names
if hasattr(model, 'names'):
    print(f"\nClass names: {model.names}")
    print(f"Number of classes: {len(model.names)}")
else:
    print("\nNo class names found")

# Print model info
print("\nModel info:")
try:
    info = model.info(verbose=True)
except Exception as e:
    print(f"Error getting info: {e}")

print("\n" + "=" * 50)
print("Testing with a sample prediction...")
print("=" * 50)

# Try to get more details
try:
    # Print available attributes
    print("\nModel attributes:")
    for attr in dir(model):
        if not attr.startswith('_'):
            try:
                value = getattr(model, attr)
                if not callable(value):
                    print(f"  {attr}: {value}")
            except:
                pass
except Exception as e:
    print(f"Error: {e}")
