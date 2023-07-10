import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.By as By
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory

//WebUI.openBrowser('')
//
//WebUI.navigateToUrl('https://admin-demo.nopcommerce.com/login?ReturnUrl=%2Fadmin%2F')
//
//WebUI.click(findTestObject('Object Repository/Page_Your store. Login/button_Log in'))
//
//WebUI.click(findTestObject('Object Repository/Page_Dashboard  nopCommerce administration/p_Content management'))
//
//WebUI.click(findTestObject('Object Repository/Page_Dashboard  nopCommerce administration/p_Message templates'))

WebDriver driver = DriverFactory.getWebDriver()
'locator count pagination'
WebElement pagination = driver.findElement(By.xpath('//ul[@class="pagination"]'))
List<WebElement> count_pagination = pagination.findElements(By.xpath('.//li'))
println(count_pagination.size())
count_pagination.get(1).click()
for(int k = 1; k <(count_pagination.size()-2);k++) {
	println(k)
	'locator table'
	WebElement table = driver.findElement(By.xpath('//table//tbody'))
	List<WebElement> rows = table.findElements(By.xpath('.//tr'))
	for (int i = 0; i < rows.size(); i++) {
		WebElement colsUsernames = rows.get(i).findElement(By.xpath('.//td[1]'))
		println("Name is: "+colsUsernames.getText())
//		for (WebElement colsUsername : colsUsernames) {
//			if(colsUsername.getText()!= "" && colsUsername.getText() != "All stores" && colsUsername.getText()!= "Edit") {
//				println("name or subject is: "+ colsUsername.getText())
//			}
//		}
	}
	'click next page'
	WebUI.click(findTestObject('Object Repository/Topic/nextpage'))
}




//for(int k = 1; k <= (count_pagination.size()-2);k++) {
//	println(k)
//	count_pagination.get(k).click()
//
//	'locator table'
//	WebElement table = driver.findElement(By.xpath('//table//tbody'))
//	List<WebElement> rows = table.findElements(By.xpath('.//tr'))
//	for (int i = 0; i < rows.size(); i++) {
//		WebElement colsUsernames = rows.get(i).findElement(By.xpath('.//td[1]'))
//		println("Name is: "+colsUsernames.getText())
////		for (WebElement colsUsername : colsUsernames) {
////			if(colsUsername.getText()!= "" && colsUsername.getText() != "All stores" && colsUsername.getText()!= "Edit") {
////				println("name or subject is: "+ colsUsername.getText())
////			}
////		}
//	}
//}
