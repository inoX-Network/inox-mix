// Komponente: FAQPage — Hilfe und häufig gestellte Fragen
import { useState } from 'react';

/** FAQ-Eintrag */
interface FAQItem {
  id: string;
  category: string;
  question: string;
  answer: string;
}

/** FAQ-Kategorien */
type FAQCategory = 'start' | 'audio' | 'fx' | 'streaming' | 'troubleshoot';

/** FAQ-Daten */
const FAQ_DATA: FAQItem[] = [
  {
    id: '1',
    category: 'start',
    question: 'Wie starte ich mit inoX-MIX?',
    answer: 'Wähle zuerst deine Audio-Geräte in den Einstellungen aus. Dann nutze Quick Calibrate für automatische Mikrofon-Kalibrierung. Die Hardware-Inputs (Cyan) sind physische Geräte, Virtual-Inputs (Orange) sind Software-Quellen.',
  },
  {
    id: '2',
    category: 'audio',
    question: 'Was ist der Unterschied zwischen A-Bus und B-Bus?',
    answer: 'A-Busse (Cyan) sind für physische Hardware-Ausgänge (Speakers, Headset). B-Busse (Orange) sind virtuelle Outputs für Software (Stream, VoIP). Du kannst jeden Input zu mehreren Bussen routen.',
  },
  {
    id: '3',
    category: 'fx',
    question: 'Wie nutze ich die FX-Chain optimal?',
    answer: 'Die FX-Chain wird von oben nach unten durchlaufen: HPF → Denoise → Gate → De-Esser → EQ → Compressor → Limiter → Auto-Gain. Aktiviere nur die Module, die du brauchst. Quick Calibrate stellt Gate und Compressor automatisch ein.',
  },
  {
    id: '4',
    category: 'streaming',
    question: 'Was macht das Ducking?',
    answer: 'Ducking reduziert automatisch die Musik-Lautstärke wenn du sprichst (Sidechain-Kompressor). Stelle Amount (Reduktion), Attack (wie schnell) und Release (wie langsam zurück) ein. Threshold bestimmt ab welchem Mikro-Pegel geduckt wird.',
  },
  {
    id: '5',
    category: 'streaming',
    question: 'Wie funktioniert der Bleeper?',
    answer: 'Der Bleeper zensiert automatisch erkannte Wörter. Modi: Beep (Ton), Mute (Stille), Noise (Rauschen), Reverse (rückwärts), Custom (eigener Sound). Du musst ihn mit "ARM" scharf schalten, sonst ist er inaktiv.',
  },
  {
    id: '6',
    category: 'troubleshoot',
    question: 'Kein Audio-Signal sichtbar?',
    answer: 'Prüfe: (1) PipeWire läuft, (2) Richtiges Eingabegerät in Einstellungen, (3) Mikro nicht gemuted, (4) Gate-Threshold nicht zu hoch. Nutze Quick Calibrate für automatische Einstellung.',
  },
  {
    id: '7',
    category: 'troubleshoot',
    question: 'Latenz zu hoch?',
    answer: 'Reduziere die Buffer Size in den Einstellungen (z.B. 128 samples statt 256). Deaktiviere nicht benötigte FX-Module. Stelle sicher, dass PipeWire mit niedriger Latenz läuft.',
  },
  {
    id: '8',
    category: 'audio',
    question: 'Was ist der Unterschied zwischen Peak und RMS?',
    answer: 'Peak (Spitze) zeigt die maximalen Ausschläge, RMS (Root Mean Square) die durchschnittliche Lautstärke. VU-Meter zeigen beide: Peak = hellere Segmente, RMS = gedimmte Segmente. Vermeide Peak über -3dB.',
  },
];

const CATEGORIES: { id: FAQCategory; label: string; icon: string }[] = [
  { id: 'start', label: 'Erste Schritte', icon: '🚀' },
  { id: 'audio', label: 'Audio', icon: '🎵' },
  { id: 'fx', label: 'FX', icon: '🎛️' },
  { id: 'streaming', label: 'Streaming', icon: '📡' },
  { id: 'troubleshoot', label: 'Fehlerbehebung', icon: '🔧' },
];

/** FAQ-Seite mit Akkordeon-Einträgen und Suchfunktion */
interface FAQPageProps {}

function FAQPage(_props: FAQPageProps) {
  const [search, setSearch] = useState('');
  const [activeCategory, setActiveCategory] = useState<FAQCategory | 'all'>('all');
  const [openItems, setOpenItems] = useState<Set<string>>(new Set());

  // Filtern nach Suche und Kategorie
  const filteredFAQ = FAQ_DATA.filter((item) => {
    const matchesSearch =
      search === '' ||
      item.question.toLowerCase().includes(search.toLowerCase()) ||
      item.answer.toLowerCase().includes(search.toLowerCase());

    const matchesCategory =
      activeCategory === 'all' || item.category === activeCategory;

    return matchesSearch && matchesCategory;
  });

  const toggleItem = (id: string) => {
    setOpenItems((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  return (
    <div className="p-6 max-w-3xl mx-auto">
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-[14px] font-extrabold uppercase tracking-wider text-cyan-500 mb-2">
          📖 Hilfe & FAQ
        </h1>
        <p className="text-[6px] text-gray-500">
          Häufig gestellte Fragen zu inoX-MIX
        </p>
      </div>

      {/* Suchfeld */}
      <div className="mb-4">
        <input
          type="text"
          placeholder="Suche nach Stichwort..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-[6px] text-gray-300 placeholder-gray-600 focus:outline-none focus:border-cyan-500"
        />
      </div>

      {/* Kategorien */}
      <div className="flex gap-2 mb-6 overflow-x-auto">
        <button
          onClick={() => setActiveCategory('all')}
          className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded whitespace-nowrap transition-colors ${
            activeCategory === 'all'
              ? 'bg-cyan-500 text-background'
              : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
          }`}
        >
          Alle
        </button>

        {CATEGORIES.map((cat) => (
          <button
            key={cat.id}
            onClick={() => setActiveCategory(cat.id)}
            className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded whitespace-nowrap transition-colors ${
              activeCategory === cat.id
                ? 'bg-cyan-500 text-background'
                : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
            }`}
          >
            {cat.icon} {cat.label}
          </button>
        ))}
      </div>

      {/* FAQ-Einträge (Akkordeon) */}
      <div className="space-y-2">
        {filteredFAQ.length === 0 ? (
          <div className="text-center py-8 text-[5px] text-gray-500">
            Keine Ergebnisse gefunden
          </div>
        ) : (
          filteredFAQ.map((item) => {
            const isOpen = openItems.has(item.id);

            return (
              <div
                key={item.id}
                className="bg-gray-800 border border-gray-700 rounded overflow-hidden"
              >
                {/* Frage (Klickbar) */}
                <button
                  onClick={() => toggleItem(item.id)}
                  className="w-full px-4 py-3 text-left flex items-center justify-between hover:bg-gray-750 transition-colors"
                >
                  <span className="text-[6px] font-semibold text-gray-200">
                    {item.question}
                  </span>
                  <span
                    className="text-[8px] text-cyan-500 transform transition-transform"
                    style={{
                      transform: isOpen ? 'rotate(180deg)' : 'rotate(0deg)',
                    }}
                  >
                    ▼
                  </span>
                </button>

                {/* Antwort (expandierbar) */}
                {isOpen && (
                  <div className="px-4 py-3 border-t border-gray-700 bg-gray-850">
                    <p className="text-[5.5px] text-gray-400 leading-relaxed">
                      {item.answer}
                    </p>
                  </div>
                )}
              </div>
            );
          })
        )}
      </div>

      {/* Footer */}
      <div className="mt-8 pt-4 border-t border-gray-800 text-center">
        <p className="text-[5px] text-gray-600">
          Weitere Hilfe benötigt? Besuche{' '}
          <a
            href="https://github.com/inox-network/inox-mix"
            target="_blank"
            rel="noopener noreferrer"
            className="text-cyan-500 hover:underline"
          >
            GitHub
          </a>{' '}
          oder öffne ein{' '}
          <a
            href="https://github.com/inox-network/inox-mix/issues"
            target="_blank"
            rel="noopener noreferrer"
            className="text-cyan-500 hover:underline"
          >
            Issue
          </a>
        </p>
      </div>
    </div>
  );
}

export default FAQPage;
