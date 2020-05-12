export const ScreenReaderOnly = ({ children }) => (
  <span>
    {children}

    {/* Nicked from https://tailwindcss.com/docs/screen-readers/ */}
    <style jsx>{`
      span {
        position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        border-width: 0;
      }
    `}</style>
  </span>
);
