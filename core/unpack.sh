#!/bin/sh
# unpack.sh - Versione con debug

BUNDLE_FILE="$1"

if [ ! -f "$BUNDLE_FILE" ]; then
    echo "File $BUNDLE_FILE non trovato"
    exit 1
fi

echo "📂 Lettura file: $BUNDLE_FILE"
echo "=================================="

current_file=""
line_num=0

while read line; do
    line_num=$((line_num + 1))
    
    # Debug: mostra le prime righe
    if [ $line_num -le 5 ]; then
        echo "DEBUG riga $line_num: $line"
    fi
    
    case "$line" in
        --\$FILE===:*)
            current_file=$(echo "$line" | cut -c12- | sed 's/^ //')
            # echo "🔍 TROVATO marker: $current_file"
            mkdir -p "$(dirname "$current_file")"
            echo "📝 Creo file: $current_file"
            > "$current_file"
            ;;
        *)
            if [ -n "$current_file" ]; then
                echo "$line" >> "$current_file"
            fi
            ;;
    esac
done < "$BUNDLE_FILE"

echo "=================================="
echo "✅ Fatto! Totale righe: $line_num"
