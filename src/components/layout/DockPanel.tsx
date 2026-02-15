// Komponente: DockPanel — dockbares/andockbares Panel-System
import { useState, useEffect, useRef } from 'react';

/** Dockbares Panel das verschoben und angedockt werden kann */
interface DockPanelProps {
  /** Panel-Titel */
  title: string;
  /** Panel-Inhalt */
  children: React.ReactNode;
  /** Initial angedockt */
  initialDocked?: boolean;
  /** Initial Position (wenn nicht angedockt) */
  initialPosition?: { x: number; y: number };
  /** Initial Größe */
  initialSize?: { width: number; height: number };
}

function DockPanel({
  title,
  children,
  initialDocked = true,
  initialPosition = { x: 100, y: 100 },
  initialSize = { width: 300, height: 400 },
}: DockPanelProps) {
  const [isDocked, setIsDocked] = useState(initialDocked);
  const [position, setPosition] = useState(initialPosition);
  const [size, setSize] = useState(initialSize);
  const [isResizing, setIsResizing] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const dragStartPos = useRef({ x: 0, y: 0 });
  const resizeStartSize = useRef({ width: 0, height: 0 });

  const handleDockToggle = () => {
    setIsDocked(!isDocked);
  };

  const handleDragStart = (e: React.MouseEvent) => {
    if (isDocked) return;
    e.preventDefault();
    setIsDragging(true);
    dragStartPos.current = {
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    };
  };

  const handleResizeStart = (e: React.MouseEvent) => {
    if (isDocked) return;
    e.preventDefault();
    e.stopPropagation();
    setIsResizing(true);
    resizeStartSize.current = {
      width: e.clientX - position.x,
      height: e.clientY - position.y,
    };
  };

  // Drag & Resize Event-Handling
  useEffect(() => {
    if (!isDragging && !isResizing) return;

    const handleMouseMove = (e: MouseEvent) => {
      if (isDragging) {
        setPosition({
          x: e.clientX - dragStartPos.current.x,
          y: e.clientY - dragStartPos.current.y,
        });
      } else if (isResizing) {
        const newWidth = Math.max(200, e.clientX - position.x);
        const newHeight = Math.max(150, e.clientY - position.y);
        setSize({ width: newWidth, height: newHeight });
      }
    };

    const handleMouseUp = () => {
      setIsDragging(false);
      setIsResizing(false);
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, isResizing, position.x, position.y]);

  // Angedockt: Normales Panel im Layout
  if (isDocked) {
    return (
      <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px]">
        {/* Header */}
        <div className="flex items-center justify-between px-3 py-2 border-b border-[rgba(255,255,255,0.05)]">
          <h3 className="text-[7px] font-bold uppercase text-gray-400 tracking-wide">
            {title}
          </h3>

          {/* Drag Handle (6 Dots) + Undock Button */}
          <div className="flex items-center gap-2">
            {/* 6 Dots Drag Handle */}
            <div className="grid grid-cols-2 gap-0.5 opacity-30 hover:opacity-100 transition-opacity cursor-move">
              <div className="w-0.5 h-0.5 rounded-full bg-gray-500" />
              <div className="w-0.5 h-0.5 rounded-full bg-gray-500" />
              <div className="w-0.5 h-0.5 rounded-full bg-gray-500" />
              <div className="w-0.5 h-0.5 rounded-full bg-gray-500" />
              <div className="w-0.5 h-0.5 rounded-full bg-gray-500" />
              <div className="w-0.5 h-0.5 rounded-full bg-gray-500" />
            </div>

            {/* Undock Button */}
            <button
              onClick={handleDockToggle}
              className="text-[5px] px-1.5 py-0.5 text-gray-500 hover:text-cyan-500 hover:bg-cyan-500/10 rounded transition-colors"
              title="Abdocken"
            >
              ↗
            </button>
          </div>
        </div>

        {/* Inhalt */}
        <div className="p-3">{children}</div>
      </div>
    );
  }

  // Schwebend: Floating Window
  return (
    <div
      className="fixed bg-inox-panel border border-cyan-500/50 rounded-[5px] shadow-2xl shadow-black/50 z-50"
      style={{
        left: `${position.x}px`,
        top: `${position.y}px`,
        width: `${size.width}px`,
        height: `${size.height}px`,
      }}
    >
      {/* Header (Draggable) */}
      <div
        className="flex items-center justify-between px-3 py-2 border-b border-cyan-500/30 bg-cyan-500/10 cursor-move select-none"
        onMouseDown={handleDragStart}
      >
        <h3 className="text-[7px] font-bold uppercase text-cyan-500 tracking-wide">
          {title}
        </h3>

        {/* Dock Button */}
        <button
          onClick={handleDockToggle}
          className="text-[5px] px-1.5 py-0.5 text-cyan-500 hover:bg-cyan-500 hover:text-background rounded transition-colors"
          title="Andocken"
        >
          ↙
        </button>
      </div>

      {/* Inhalt (Scrollable) */}
      <div className="p-3 overflow-auto" style={{ height: `calc(100% - 36px)` }}>
        {children}
      </div>

      {/* Resize Handle (unten-rechts) */}
      <div
        className="absolute bottom-0 right-0 w-4 h-4 cursor-se-resize"
        onMouseDown={handleResizeStart}
      >
        <div className="absolute bottom-1 right-1 w-2 h-2 border-r-2 border-b-2 border-cyan-500/50" />
      </div>

      {/* Drag/Resize Indicator */}
      {(isDragging || isResizing) && (
        <div className="absolute inset-0 bg-cyan-500/10 pointer-events-none rounded-[5px]" />
      )}
    </div>
  );
}

export default DockPanel;
