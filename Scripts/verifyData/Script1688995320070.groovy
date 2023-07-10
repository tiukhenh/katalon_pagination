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
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testdata.TestData

//WebUI.openBrowser('')
//
//WebUI.navigateToUrl('https://admin-demo.nopcommerce.com/login?ReturnUrl=%2Fadmin%2F')
//
//WebUI.click(findTestObject('Object Repository/Page_Your store. Login/button_Log in'))
//
//WebUI.click(findTestObject('Object Repository/Page_Dashboard  nopCommerce administration/p_Content management'))
//
//WebUI.click(findTestObject('Object Repository/Page_Dashboard  nopCommerce administration/p_Message templates'))

TestData data = TestDataFactory.findTestData("Data Files/Expect Data")
//List<String> myList = data.getColumnValues("name")
List<String> expectData = data.allData
println(expectData)

WebDriver driver = DriverFactory.getWebDriver()
'locator table'
WebElement table = driver.findElement(By.xpath('//table//tbody'))
List<WebElement> rows = table.findElements(By.xpath('.//tr'))
int countData =0;
for (int i = 0; i < rows.size(); i++) {
    List<WebElement> cols = rows.get(i).findElements(By.xpath('.//td[1]'))
	println(expectData.size())
    for (WebElement col : cols) {
		for(j=0;j<expectData.size();j++) {		
//			println(expectData[j][0])
			'expectData[j][0]: ExpectData is 2-dimensional array'
			if(col.getText()== expectData[j][0]) {
				countData++
			}
		}
    }
}
if(countData==expectData.size()) {
	println("Actual data them sam expect data")
}




