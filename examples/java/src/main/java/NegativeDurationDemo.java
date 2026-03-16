import java.io.FileOutputStream;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.time.Duration;
import org.apache.fory.Fory;
import org.apache.fory.config.Language;
import org.apache.fory.logging.LoggerFactory;

public class NegativeDurationDemo {
  static final String OUTPUT_FILE = "/tmp/negative_duration.txt";

  public static void main(String[] args) throws Exception {
    LoggerFactory.disableLogging();
    Fory fory =
        Fory.builder()
            .withLanguage(Language.XLANG)
            .requireClassRegistration(false)
            .build();
    Duration negativeDuration = Duration.ofSeconds(-1);
    byte[] bytes = fory.serialize(negativeDuration);
    Files.write(Paths.get(OUTPUT_FILE), bytes);
    System.out.println("Java  | bytes (hex): " + toHex(bytes));
  }

  private static String toHex(byte[] bytes) {
    StringBuilder sb = new StringBuilder();
    for (byte b : bytes) {
      sb.append(String.format("%02x ", b));
    }
    return sb.toString().trim();
  }
}
