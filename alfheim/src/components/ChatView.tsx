import React, { useState } from 'react';
import { View, Text, TextInput, Button, StyleSheet } from 'react-native';
import { PlatformService } from '../services/platformService';

export function ChatView() {
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<string[]>([]);
  const platformService = new PlatformService();

  const handleSend = async () => {
    if (!message.trim()) return;

    setMessages([...messages, `You: ${message}`]);
    
    try {
      await platformService.initialize();
      const response = await platformService.processRequest({
        text: message,
      });
      setMessages([...messages, `You: ${message}`, `Assistant: ${response.text}`]);
    } catch (error) {
      setMessages([...messages, `You: ${message}`, `Error: ${error}`]);
    }

    setMessage('');
  };

  return (
    <View style={styles.container}>
      <View style={styles.messages}>
        {messages.map((msg, idx) => (
          <Text key={idx} style={styles.message}>{msg}</Text>
        ))}
      </View>
      <View style={styles.inputContainer}>
        <TextInput
          style={styles.input}
          value={message}
          onChangeText={setMessage}
          placeholder="Type a message..."
        />
        <Button title="Send" onPress={handleSend} />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 16,
  },
  messages: {
    flex: 1,
    marginBottom: 16,
  },
  message: {
    marginBottom: 8,
  },
  inputContainer: {
    flexDirection: 'row',
    gap: 8,
  },
  input: {
    flex: 1,
    borderWidth: 1,
    borderColor: '#ccc',
    padding: 8,
    borderRadius: 4,
  },
});
