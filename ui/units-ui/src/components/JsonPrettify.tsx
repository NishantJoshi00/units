import React from 'react';

interface JsonPrettifierProps {
  output: string | object | unknown;
  className?: string;
}

export const JsonPrettifier: React.FC<JsonPrettifierProps> = ({ 
  output, 
  className = ''
}) => {
  const parseNestedJson = (data: unknown): unknown => {
    if (typeof data === 'string') {
      try {
        const parsed = JSON.parse(data);
        return parseNestedJson(parsed); // Recursively parse the result
      } catch {
        return data; // If it's not valid JSON, return the original string
      }
    } else if (Array.isArray(data)) {
      return data.map(item => parseNestedJson(item));
    } else if (typeof data === 'object' && data !== null) {
      const result: Record<string, unknown> = {};
      for (const [key, value] of Object.entries(data)) {
        result[key] = parseNestedJson(value);
      }
      return result;
    }
    return data;
  };

  const formatJson = (data: unknown): string => {
    try {
      // If the input is a string, parse it first
      const initialParse = typeof data === 'string' ? JSON.parse(data) : data;
      // Then recursively parse any nested JSON strings
      const fullyParsed = parseNestedJson(initialParse);
      return JSON.stringify(fullyParsed, null, 2);
    } catch (error) {
      console.error('JSON parsing error:', error);
      return 'Invalid JSON';
    }
  };

  return (
    <div className={`p-4 bg-gray-100 rounded-md ${className}`}>
      <pre className="whitespace-pre-wrap font-mono text-sm overflow-x-auto">
        {formatJson(output)}
      </pre>
    </div>
  );
};

