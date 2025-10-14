import React, { useState, useRef } from 'react';
import './ImageUpload.css';

interface ImageUploadProps {
  onImageUpload: (imageUrl?: string, imageFile?: File) => void;
  loading: boolean;
}

const ImageUpload: React.FC<ImageUploadProps> = ({ onImageUpload, loading }) => {
  const [imageUrl, setImageUrl] = useState('');
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [previewUrl, setPreviewUrl] = useState<string>('');
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file && file.type.startsWith('image/')) {
      setSelectedFile(file);
      setImageUrl('');
      const reader = new FileReader();
      reader.onload = () => {
        setPreviewUrl(reader.result as string);
      };
      reader.readAsDataURL(file);
    }
  };

  const handleUrlSubmit = () => {
    if (imageUrl.trim()) {
      setSelectedFile(null);
      setPreviewUrl(imageUrl);
      onImageUpload(imageUrl);
    }
  };

  const handleFileSubmit = () => {
    if (selectedFile) {
      onImageUpload(undefined, selectedFile);
    }
  };

  const handleDragOver = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
  };

  const handleDrop = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    const file = event.dataTransfer.files[0];
    if (file && file.type.startsWith('image/')) {
      setSelectedFile(file);
      setImageUrl('');
      const reader = new FileReader();
      reader.onload = () => {
        setPreviewUrl(reader.result as string);
      };
      reader.readAsDataURL(file);
    }
  };

  const clearSelection = () => {
    setSelectedFile(null);
    setImageUrl('');
    setPreviewUrl('');
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  return (
    <div className="image-upload">
      <h2>Find Similar Products</h2>
      <p>Upload an image or enter an image URL to find visually similar products</p>
      
      <div className="upload-options">
        {/* File Upload */}
        <div className="upload-section">
          <h3>Upload Image</h3>
          <div
            className={`drop-zone ${previewUrl && selectedFile ? 'has-image' : ''}`}
            onDragOver={handleDragOver}
            onDrop={handleDrop}
            onClick={() => fileInputRef.current?.click()}
          >
            <input
              ref={fileInputRef}
              type="file"
              accept="image/*"
              onChange={handleFileChange}
              style={{ display: 'none' }}
            />
            {previewUrl && selectedFile ? (
              <img src={previewUrl} alt="Preview" className="preview-image" />
            ) : (
              <div className="drop-zone-content">
                <p>📁 Drag & drop an image here or click to browse</p>
                <small>Supports JPG, PNG, WebP</small>
              </div>
            )}
          </div>
          {selectedFile && (
            <div className="file-actions">
              <button 
                onClick={handleFileSubmit} 
                disabled={loading}
                className="search-btn"
              >
                {loading ? 'Searching...' : 'Search Similar Products'}
              </button>
              <button onClick={clearSelection} className="clear-btn">
                Clear
              </button>
            </div>
          )}
        </div>

        {/* URL Input */}
        <div className="upload-section">
          <h3>Or Enter Image URL</h3>
          <div className="url-input-group">
            <input
              type="url"
              value={imageUrl}
              onChange={(e) => setImageUrl(e.target.value)}
              placeholder="https://example.com/image.jpg"
              className="url-input"
            />
            <button 
              onClick={handleUrlSubmit} 
              disabled={!imageUrl.trim() || loading}
              className="search-btn"
            >
              {loading ? 'Searching...' : 'Search'}
            </button>
          </div>
          {previewUrl && !selectedFile && (
            <div className="preview-section">
              <img src={previewUrl} alt="Preview" className="preview-image" />
              <button onClick={clearSelection} className="clear-btn">
                Clear
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default ImageUpload;