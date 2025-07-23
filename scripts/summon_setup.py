from PIL import Image
import os

def png_to_webp(folder_path='assets/summon_images'):
    # Walk through the directory
    for root, dirs, files in os.walk(folder_path):
        for file in files:
            # Check for PNG files
            if file.endswith('.png'):
                png_file_path = os.path.join(root, file)
                
                # Define the new file name by changing the extension to .webp
                webp_file_path = os.path.splitext(png_file_path)[0] + '.webp'
                
                # Check if the WEBP file already exists
                if os.path.isfile(webp_file_path):
                    print(f"Skipping {png_file_path}: already converted to WEBP.")
                    continue

                # Load the PNG image
                try:
                    with Image.open(png_file_path) as img:
                        # Convert and save the image as WEBP
                        img.save(webp_file_path, 'WEBP')
                        print(f"Converted {png_file_path} to {webp_file_path}")
                except Exception as e:
                    print(f"Failed to convert {png_file_path}: {e}")

def generate_pool(folder_path='assets/summon_images', output_file='src/summon/pool.rs'):
    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    # Buffer to store the entire Rust file content
    rust_code = ""

    # Walk through subdirectories
    for root, _, files in os.walk(folder_path):
        if root == folder_path:
            continue  # Skip the root directory

        # Get variable name from folder name
        folder_name = os.path.basename(root)
        var_name = folder_name.upper().replace('-', '_')  # Ensure it's a valid Rust identifier

        rust_code += f"pub static {var_name}: &[&str] = &[\n"

        for file in sorted(files):
            if file.endswith('.webp'):
                rust_code += f'    "{file}",\n'

        rust_code += "];\n\n"

    # Write to the single Rust file
    with open(output_file, 'w') as f:
        f.write(rust_code)

    print(f"Rust file generated at {output_file}")

def delete_png_files(folder_path='assets/summon_images'):
    # Walk through the directory
    for root, dirs, files in os.walk(folder_path):
        for file in files:
            if file.endswith('.png'):
                png_file_path = os.path.join(root, file)
                try:
                    os.remove(png_file_path)
                    print(f"Deleted: {png_file_path}")
                except Exception as e:
                    print(f"Failed to delete {png_file_path}: {e}")

def convert_images_to_resolution(input_folder='assets/summon_images', desired_resolution=(256, 256)):
    # Iterate through all files in the input folder
    for root, _, files in os.walk(input_folder):
        for file in files:
            # Check if the file is an image
            if file.lower().endswith(('.webp')):
                # Create the full path to the image
                input_path = os.path.join(root, file)
                
                # Open the image
                with Image.open(input_path) as img:
                    # Resize the image
                    img_resized = img.resize(desired_resolution, Image.LANCZOS)
                    
                    # Save the resized image back to the original path
                    img_resized.save(input_path)
                    print(f'Resized and saved image: {input_path}')

# Useful refresh operation - will make large changes so run only when needed
def refresh():
    png_to_webp()
    delete_png_files()
    convert_images_to_resolution()
    generate_pool()

refresh()